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
usage() { printf 'usage: release-dmg.sh --check | NEW_OUTPUT_DIRECTORY\n'; }
if [[ $# != 1 ]]; then usage >&2; exit 2; fi
case "$1" in
  -h|--help) usage; exit 0 ;;
  --check) ;;
  -*) usage >&2; exit 2 ;;
  *) [[ ! -e "$1" && ! -L "$1" ]] || fail 'Choose a new output directory.' ;;
esac

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

case "$1" in
  /*) output="$1" ;;
  *) output="$PWD/$1" ;;
esac
mkdir -- "$output"
output="$(cd "$output" && pwd -P)"
work="$(mktemp -d "$output/.work.XXXXXXXX")"
mount=
mounted=0
cleanup() {
  local result=$?
  if [[ "$mounted" = 1 ]] && ! hdiutil detach "$mount" -quiet; then
    printf 'error: detach %s before removing %s\n' "$mount" "$work" >&2
    exit 1
  fi
  [[ -z "$mount" ]] || rmdir -- "$mount"
  rm -rf -- "$work"
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

notarize() {
  local artifact="$1" label="$2" id status tool_result=0
  local receipt="$output/notary-$label.json"
  local profile=(--keychain-profile "$MACOS_NOTARY_PROFILE" ${notary_keychain[@]+"${notary_keychain[@]}"})
  xcrun notarytool submit "$artifact" "${profile[@]}" --wait --timeout 45m --output-format json > "$receipt" || tool_result=$?
  id="$(plutil -extract id raw -o - "$receipt" 2>/dev/null)" || id=
  status="$(plutil -extract status raw -o - "$receipt" 2>/dev/null)" || status=
  [[ "$id" =~ ^[[:xdigit:]]{8}(-[[:xdigit:]]{4}){3}-[[:xdigit:]]{12}$ ]] || fail "Invalid or missing notarization ID for $label. See $receipt."
  if [[ "$tool_result" != 0 || "$status" != Accepted ]]; then
    xcrun notarytool log "$id" "${profile[@]}" "$output/notary-$label-log.json" || true
    fail "Apple did not accept $label. See $receipt."
  fi
  printf 'PASS notarization %s %s\n' "$label" "$id"
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

"$HERE/local-app.sh" --tauri --release "$work/bundle"
app="$work/bundle/$APP_NAME"
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

sign --options runtime "$app"
verify_app "$app"
ditto -c -k --keepParent "$app" "$work/app.zip"
notarize "$work/app.zip" app
staple_app "$app"
retry xcrun stapler validate "$app"
verify_app "$app"

mkdir -m 755 "$work/stage"
ditto "$app" "$work/stage/$APP_NAME"
ln -s /Applications "$work/stage/Applications"
filename="ShiftAnything-$version-macos-$release_arch.dmg"
dmg="$work/$filename"
hdiutil create -srcfolder "$work/stage" -volname 'Sh*ftAnything' -fs HFS+ -format UDZO "$dmg"
sign --identifier "$BUNDLE_ID.disk-image" "$dmg"
codesign --verify --strict "$dmg"
verify_signature "$dmg" dmg
notarize "$dmg" dmg
retry xcrun stapler staple "$dmg"
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
ditto "$mount/$APP_NAME" "$work/$APP_NAME"
verify_app "$work/$APP_NAME"
retry xcrun stapler validate "$work/$APP_NAME"
retry assess --type execute "$work/$APP_NAME"
check_bundle_version "$work/$APP_NAME" "$version"
hdiutil detach "$mount" -quiet
mounted=0
rmdir -- "$mount"
mount=

(cd "$work" && shasum -a 256 "$filename" > "$output/SHA256SUMS.txt")
chmod 644 "$dmg"
mv -- "$dmg" "$output/$filename"
printf 'PASS notarized and verified macOS DMG: %s\n' "$output/$filename"
