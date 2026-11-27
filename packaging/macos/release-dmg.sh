#!/usr/bin/env bash
# Sign, notarize and verify the production Tauri bundle, then hand off the DMG locally.
set -euo pipefail
umask 077
export LC_ALL=C

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=check-bundle.sh
. "$HERE/check-bundle.sh"
APP_NAME='Sh*ftAnything.app'
BUNDLE_ID=it.promptica.shiftanything

fail() { printf 'error: %s\n' "$*" >&2; exit 1; }
usage() { printf 'usage: release-dmg.sh --check | NEW_OUTPUT_DIRECTORY | --resume OUTPUT_DIRECTORY\n'; }
resume=0
case "${1:-}" in
  -h|--help) usage; exit 0 ;;
  --resume) [[ $# = 2 ]] || { usage >&2; exit 2; }; resume=1; shift ;;
  --check) [[ $# = 1 ]] || { usage >&2; exit 2; } ;;
  -*) usage >&2; exit 2 ;;
esac
[[ $# = 1 ]] || { usage >&2; exit 2; }
if [[ "$1" != --check ]]; then
  if [[ "$resume" = 1 ]]; then
    [[ ! -L "$1" && -f "$1/.notary-resume-v1" && ! -L "$1/.notary-resume-v1" ]] || fail 'This directory does not contain resumable release state.'
  else
    [[ ! -e "$1" && ! -L "$1" ]] || fail 'Choose a new output directory or use --resume.'
  fi
fi

[[ "$(uname -s)" = Darwin ]] || fail 'Run this script on macOS with Xcode.'
[[ "${MACOS_SIGN_IDENTITY:-}" =~ ^[[:xdigit:]]{40}$ ]] || fail 'Set MACOS_SIGN_IDENTITY to the SHA-1 of a Developer ID Application identity.'
[[ "${APPLE_TEAM_ID:-}" =~ ^[A-Z0-9]{10}$ ]] || fail 'Set APPLE_TEAM_ID to the Apple Developer team ID.'
[[ "${MACOS_NOTARY_PROFILE:-}" =~ ^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$ ]] || fail 'Set MACOS_NOTARY_PROFILE to a verified notarytool profile name.'

identity="$(printf '%s' "$MACOS_SIGN_IDENTITY" | tr '[:lower:]' '[:upper:]')"
sign_keychain=()
notary_keychain=()
if [[ -n "${MACOS_SIGN_KEYCHAIN:-}" ]]; then
  [[ "$MACOS_SIGN_KEYCHAIN" = /* && -f "$MACOS_SIGN_KEYCHAIN" ]] || fail 'MACOS_SIGN_KEYCHAIN must name an existing keychain by absolute path.'
  sign_keychain=(--keychain "$MACOS_SIGN_KEYCHAIN")
fi
if [[ -n "${MACOS_NOTARY_KEYCHAIN:-}" ]]; then
  [[ "$MACOS_NOTARY_KEYCHAIN" = /* && -f "$MACOS_NOTARY_KEYCHAIN" ]] || fail 'MACOS_NOTARY_KEYCHAIN must name an existing keychain by absolute path.'
  notary_keychain=(--keychain "$MACOS_NOTARY_KEYCHAIN")
fi

identities="$(security find-identity -v -p codesigning ${MACOS_SIGN_KEYCHAIN:+"$MACOS_SIGN_KEYCHAIN"})"
printf '%s\n' "$identities" | awk -v identity="$identity" -v team="$APPLE_TEAM_ID" '
  BEGIN { suffix = "(" team ")\"" }
  $2 == identity && index($0, "\"Developer ID Application: ") &&
    substr($0, length($0) - length(suffix) + 1) == suffix { matching++ }
  END { exit matching != 1 }' || fail 'No valid Developer ID Application identity matches the supplied fingerprint and team.'
# Look up only the profile entry. The security tool must not request password data.
security find-generic-password -l com.apple.gke.notary.tool -a "com.apple.gke.notary.tool.saved-creds.$MACOS_NOTARY_PROFILE" \
  ${MACOS_NOTARY_KEYCHAIN:+"$MACOS_NOTARY_KEYCHAIN"} >/dev/null 2>&1 \
  || fail 'The notarytool profile was not found in the selected keychain search scope.'
xcrun --find notarytool >/dev/null || fail 'Xcode notarytool is unavailable.'
xcrun --find stapler >/dev/null || fail 'Xcode stapler is unavailable.'
[[ "$(spctl --status)" = 'assessments enabled' ]] || fail 'Enable Gatekeeper assessments on this Mac before verifying releases.'
printf 'PASS local release preflight\n'
[[ "$1" != --check ]] || exit 0

if [[ "$resume" = 0 ]]; then
  mkdir -- "$1"
  touch "$1/.notary-resume-v1"
fi
output="$(cd "$1" && pwd -P)"
# Keep one local writer per release; lockf releases its advisory lock on process exit.
if [[ "${SHIFTANYTHING_RELEASE_LOCKED:-}" != "$output" ]]; then
  exec /usr/bin/lockf -k -t 0 "$output/.release-lock" /usr/bin/env SHIFTANYTHING_RELEASE_LOCKED="$output" "$HERE/release-dmg.sh" --resume "$output"
fi
work="$output/.work"
[[ ! -L "$work" ]] || fail 'Unexpected release work directory symlink.'
mount=
mounted=0
cleanup() {
  local result=$?
  if [[ "$mounted" = 1 ]] && ! hdiutil detach "$mount" -quiet; then
    printf 'error: detach %s before continuing %s\n' "$mount" "$output" >&2
    exit 1
  fi
  [[ -z "$mount" ]] || rmdir -- "$mount"
  exit "$result"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

sign() (
  umask 022
  codesign --force --sign "$identity" --timestamp ${sign_keychain[@]+"${sign_keychain[@]}"} "$@"
)

staple_app() (
  umask 022
  retry xcrun stapler staple "$1"
)

submission_id() {
  local id
  id="$(plutil -extract id raw -o - "$1" 2>/dev/null)" || return 1
  [[ "$id" =~ ^[[:xdigit:]]{8}(-[[:xdigit:]]{4}){3}-[[:xdigit:]]{12}$ ]] || return 1
  printf '%s\n' "$id"
}

notarize() {
  local artifact="$1" label="$2" id status wait_time="${MACOS_NOTARY_WAIT:-45m}"
  local submission="$output/notary-$label-submission.json" receipt="$output/notary-$label.json"
  local digest="$work/$label-submitted.sha256"
  local profile=(--keychain-profile "$MACOS_NOTARY_PROFILE" ${notary_keychain[@]+"${notary_keychain[@]}"})
  [[ "$wait_time" =~ ^[1-9][0-9]*[smh]?$ ]] || fail 'MACOS_NOTARY_WAIT must be a positive duration.'
  if [[ ! -e "$submission" && ! -e "$digest" ]]; then
    shasum -a 256 "$artifact" > "$digest"
    xcrun notarytool submit "$artifact" "${profile[@]}" --no-wait --output-format json > "$submission" || true
  fi
  if [[ ! -s "$digest" ]] || ! shasum -a 256 -c "$digest" >/dev/null; then
    fail "The preserved $label payload does not match its submission. See $digest."
  fi
  id="$(submission_id "$submission")" || fail "No valid Apple ID returned for $label. Check the existing request and receipt $submission before retrying."
  status="$(plutil -extract status raw -o - "$receipt" 2>/dev/null)" || status=
  # A saved receipt applies only to its matching Apple submission and unchanged payload.
  if [[ -f "$receipt" && "$(submission_id "$receipt")" != "$id" ]]; then
    fail "Unexpected saved Apple receipt for $label."
  fi
  if [[ -z "$status" || "$status" = 'In Progress' ]]; then
    if ! xcrun notarytool wait "$id" "${profile[@]}" --timeout "$wait_time" --output-format json > "$receipt.wait"; then
      if ! xcrun notarytool info "$id" "${profile[@]}" --output-format json > "$receipt.wait"; then
        printf 'Apple receipt retained for %s (%s). Resume with --resume %s.\n' "$label" "$id" "$output" >&2
        return 75
      fi
    fi
    [[ "$(submission_id "$receipt.wait")" = "$id" ]] || fail "Unexpected Apple receipt for $label."
    status="$(plutil -extract status raw -o - "$receipt.wait")" || fail "Missing Apple status for $label."
    mv -- "$receipt.wait" "$receipt"
  fi
  case "$status" in
    Accepted) printf 'PASS notarization %s %s\n' "$label" "$id" ;;
    'In Progress') printf 'Apple is processing %s (%s). Resume with --resume %s.\n' "$label" "$id" "$output" >&2; return 75 ;;
    *) xcrun notarytool log "$id" "${profile[@]}" "$output/notary-$label-log.json" || true
       fail "Apple did not accept $label. See $receipt." ;;
  esac
}

retry() {
  local attempt
  for attempt in 1 2 3; do
    "$@" && return 0
    [[ "$attempt" = 3 ]] || sleep 10
  done
  return 1
}

assess() {
  local result
  if ! result="$(spctl --assess --ignore-cache --no-cache -vv "$@" 2>&1)"; then
    printf '%s\n' "$result" >&2
    return 1
  fi
  printf '%s\n' "$result"
  printf '%s\n' "$result" | grep -qx 'source=Notarized Developer ID'
}

verify_signature() {
  local file="$1" kind="$2" details
  shift 2
  details="$(codesign --display --verbose=4 "$@" "$file" 2>&1)" || fail "Cannot read the $kind signature."
  printf '%s\n' "$details" | grep -q '^Authority=Developer ID Application: ' || fail "Wrong $kind certificate type."
  printf '%s\n' "$details" | grep -qx "TeamIdentifier=$APPLE_TEAM_ID" || fail "Wrong $kind signing team."
  printf '%s\n' "$details" | grep -q '^Timestamp=.' || fail "Missing $kind secure timestamp."
  if [[ "$kind" = app ]]; then
    printf '%s\n' "$details" | grep -Eq '^CodeDirectory .*flags=0x[[:xdigit:]]+\([^)]*runtime' || fail 'The app lacks hardened runtime.'
  fi
}

verify_app() {
  local bundle="$1" arch
  check_bundle_modes "$bundle"
  codesign --verify --strict --deep --all-architectures "$bundle"
  for arch in "${architectures[@]}"; do
    verify_signature "$bundle" app --arch "$arch"
  done
}

released=("$output"/ShiftAnything-*-macos-*.dmg)
if [[ -f "${released[0]}" ]]; then
  [[ ${#released[@]} = 1 ]] || fail 'Unexpected number of completed release images.'
  completed="${released[0]}"
  [[ "$(cat "$output/SHA256SUMS.txt")" = "$(cd "$output" && shasum -a 256 "${completed##*/}")" ]] || fail 'The completed release checksum does not match.'
  codesign --verify --strict "$completed"
  verify_signature "$completed" dmg
  retry xcrun stapler validate "$completed"
  retry assess --type open --context context:primary-signature "$completed"
  printf 'PASS verified existing macOS DMG: %s\n' "${released[0]}"
  exit 0
fi
mkdir -p -- "$work"
app="$work/bundle/$APP_NAME"
if [[ ! -d "$app" ]]; then
  build="$(mktemp -d "$work/build.XXXXXXXX")"
  "$HERE/local-app.sh" --tauri --release "$build/bundle"
  sign --options runtime "$build/bundle/$APP_NAME"
  mv -- "$build/bundle" "$work/bundle"
  rmdir -- "$build"
fi
executable="$app/Contents/MacOS/ShiftAnything"
version="$(plutil -extract CFBundleShortVersionString raw -o - "$app/Contents/Info.plist")"
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || fail 'The app version is not a three-part release version.'
[[ "$(plutil -extract CFBundleIdentifier raw -o - "$app/Contents/Info.plist")" = "$BUNDLE_ID" ]] || fail 'Unexpected app bundle identifier.'
lipo "$executable" -verify_arch "$(uname -m)"
archs="$(lipo -archs "$executable")"
case "$archs" in
  arm64|x86_64) release_arch="$archs" ;;
  'arm64 x86_64'|'x86_64 arm64') release_arch=universal ;;
  *) fail 'Unexpected app architecture list.' ;;
esac
read -r -a architectures <<< "$archs"

filename="ShiftAnything-$version-macos-$release_arch.dmg"
verify_app "$app"
if [[ ! -f "$work/app.zip" ]]; then
  [[ ! -e "$output/notary-app-submission.json" && ! -e "$work/app-submitted.sha256" ]] || fail 'The original app archive is missing.'
  ditto -c -k --keepParent "$app" "$work/app.tmp.zip"
  mv -- "$work/app.tmp.zip" "$work/app.zip"
fi
notarize "$work/app.zip" app
xcrun stapler validate "$app" >/dev/null 2>&1 || staple_app "$app"
retry xcrun stapler validate "$app"
verify_app "$app"
retry assess --type execute "$app"

mkdir -p -- "$work/unstapled"
source_dmg="$work/unstapled/$filename"
if [[ ! -f "$source_dmg" ]]; then
  [[ ! -e "$output/notary-dmg-submission.json" && ! -e "$work/dmg-submitted.sha256" ]] || fail 'The original signed DMG is missing.'
  stage="$(mktemp -d "$work/stage.XXXXXXXX")"
  chmod 755 "$stage"
  ditto "$app" "$stage/$APP_NAME"
  ln -s /Applications "$stage/Applications"
  hdiutil create -srcfolder "$stage" -volname 'Sh*ftAnything' -fs HFS+ -format UDZO "$stage/build.dmg"
  sign --identifier "$BUNDLE_ID.disk-image" "$stage/build.dmg"
  codesign --verify --strict "$stage/build.dmg"
  verify_signature "$stage/build.dmg" dmg
  mv -- "$stage/build.dmg" "$source_dmg"
fi
codesign --verify --strict "$source_dmg"
verify_signature "$source_dmg" dmg
notarize "$source_dmg" dmg
# Stapling changes the image bytes. Keep the exact submitted image for retries.
dmg="$work/$filename"
cp -- "$source_dmg" "$dmg"
(umask 022; retry xcrun stapler staple "$dmg")
retry xcrun stapler validate "$dmg"
codesign --verify --strict "$dmg"
verify_signature "$dmg" dmg
retry assess --type open --context context:primary-signature "$dmg"
hdiutil verify "$dmg"

mount="$(mktemp -d /private/tmp/shiftanything-release.XXXXXXXX)"
hdiutil attach "$dmg" -readonly -nobrowse -noautoopen -mountpoint "$mount"
mounted=1
[[ "$(readlink "$mount/Applications")" = /Applications ]] || fail 'Missing Applications shortcut in the mounted image.'
verify_app "$mount/$APP_NAME"
verified="$(mktemp -d "$work/verified.XXXXXXXX")"
ditto "$mount/$APP_NAME" "$verified/$APP_NAME"
verify_app "$verified/$APP_NAME"
retry xcrun stapler validate "$verified/$APP_NAME"
retry assess --type execute "$verified/$APP_NAME"
check_bundle_version "$verified/$APP_NAME" "$version"
hdiutil detach "$mount" -quiet
mounted=0
rmdir -- "$mount"
mount=

(cd "$work" && shasum -a 256 "$filename" > "$output/SHA256SUMS.txt")
chmod 644 "$dmg"
mv -- "$dmg" "$output/$filename"
rm -rf -- "$work"
printf 'PASS notarized and verified macOS DMG: %s\n' "$output/$filename"
