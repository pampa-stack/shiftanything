#!/usr/bin/env bash
# Create signed updater assets locally from the verified release DMG and DEB.
set -euo pipefail
umask 077
export LC_ALL=C
unset TAURI_SIGNING_PRIVATE_KEY TAURI_SIGNING_PRIVATE_KEY_PASSWORD

usage() {
  printf '%s\n' 'Usage: package-tauri-updater.sh VERSION DMG DMG_SHA256 DEB DEB_SHA256 RELEASE_OWNER/REPO NEW_OUTPUT_DIRECTORY' \
    'Requires APPLE_TEAM_ID and a preconfigured TAURI_SIGNING_PRIVATE_KEY_PATH.' \
    'If set, SHIFT_LAUNCHER_UPDATE_PUBKEY must match both updater signatures.'
}
fail() { printf 'error: %s\n' "$*" >&2; exit 1; }
if [[ $# = 1 && ( "$1" = --help || "$1" = -h ) ]]; then usage; exit 0; fi
[[ $# = 7 ]] || { usage >&2; exit 2; }
version=$1
input_dmg=$2
expected_dmg_hash=$3
input_deb=$4
expected_deb_hash=$5
repo=$6
output=$7
[[ "$version" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]] || fail 'Expected a three-part stable version.'
[[ "$expected_dmg_hash" =~ ^[a-f0-9]{64}$ && "$expected_deb_hash" =~ ^[a-f0-9]{64}$ ]] || fail 'Provide both release receipt SHA-256 values in lowercase.'
[[ "$repo" =~ ^[A-Za-z0-9]([A-Za-z0-9-]{0,37}[A-Za-z0-9])?/[A-Za-z0-9][A-Za-z0-9._-]{0,99}$ ]] \
  || fail 'Provide the approved GitHub release owner and repository as OWNER/REPO.'
[[ "$(uname -s)" = Darwin && "$(uname -m)" = arm64 ]] || fail 'Run this tool on macOS Apple Silicon.'
[[ -f "$input_dmg" && -f "$input_deb" ]] || fail 'Both packages must exist.'
[[ "${APPLE_TEAM_ID:-}" =~ ^[A-Z0-9]{10}$ ]] || fail 'Configure the Apple Developer signing team.'
[[ "${TAURI_SIGNING_PRIVATE_KEY_PATH:-}" = /* && -f "$TAURI_SIGNING_PRIVATE_KEY_PATH" && -r "$TAURI_SIGNING_PRIVATE_KEY_PATH" ]] \
  || fail 'Configure the existing Tauri signing key file by absolute path.'
[[ ! -e "$output" && ! -L "$output" ]] || fail 'Choose a fresh output directory.'
[[ "$(/usr/sbin/spctl --status)" = 'assessments enabled' ]] || fail 'Gatekeeper assessments are required.'
if command -v tauri >/dev/null; then
  cli=(tauri)
else
  cli=(cargo tauri)
fi
sign_help="$("${cli[@]}" signer sign --help 2>/dev/null)" || fail 'Install the official Tauri CLI.'
[[ "$sign_help" = *--app-version* ]] || fail 'Use a Tauri CLI that supports version-bound signatures.'

app_name='Sh*ftAnything.app'
app_id=it.promptica.shiftanything
mac_name="ShiftAnything-$version-macos-arm64.app.tar.gz"
linux_name="ShiftAnything-$version-linux-amd64.deb"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
# shellcheck source=macos/check-bundle.sh
. "$here/macos/check-bundle.sh"
parent="$(cd "$(dirname -- "$output")" && pwd -P)"
case "$output" in */) fail 'Output must name a new directory without a trailing slash.' ;; esac
output="$parent/$(basename -- "$output")"
work="$(mktemp -d "$parent/.shiftanything-updater.XXXXXXXX")"
mount="$work/mount"
mounted=0
cleanup() {
  local result=$?
  if [[ "$mounted" = 1 ]] && ! hdiutil detach "$mount" -quiet; then
    printf 'error: detach %s before cleaning temporary files\n' "$mount" >&2
    exit 1
  fi
  rm -rf -- "$work"
  exit "$result"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
mkdir "$mount" "$work/extracted" "$work/assets"
assets="$work/assets"
dmg="$work/source.dmg"
deb="$assets/$linux_name"
cp -- "$input_dmg" "$dmg"
cp -- "$input_deb" "$deb"
sha256() { shasum -a 256 < "$1" | awk '{print $1}'; }
[[ "$(sha256 "$dmg")" = "$expected_dmg_hash" && "$(sha256 "$deb")" = "$expected_deb_hash" ]] \
  || fail 'A package does not match its release receipt.'

assess() {
  local result
  result="$(/usr/sbin/spctl --assess --ignore-cache --no-cache -vv "$@" 2>&1)" || return 1
  grep -qx 'source=Notarized Developer ID' <<< "$result"
}
verify_signature() {
  local file=$1 identifier=$2 details requirement
  requirement="=anchor apple generic and identifier \"$identifier\" and certificate 1[field.1.2.840.113635.100.6.2.6] exists and certificate leaf[field.1.2.840.113635.100.6.1.13] exists and certificate leaf[subject.OU] = \"$APPLE_TEAM_ID\""
  codesign --verify --strict --all-architectures -R "$requirement" "$file"
  details="$(codesign --display --verbose=4 "$file" 2>&1)"
  grep -q '^Timestamp=.' <<< "$details"
  if [[ "$identifier" = "$app_id" ]]; then
    grep -Eq '^CodeDirectory .*flags=0x[[:xdigit:]]+\([^)]*runtime' <<< "$details"
  fi
}
verify_app() {
  local app=$1 plist="$1/Contents/Info.plist"
  [[ -d "$app" && ! -L "$app" ]] || fail 'The expected application bundle is missing.'
  [[ "$(plutil -extract CFBundleIdentifier raw -o - "$plist")" = "$app_id" \
    && "$(plutil -extract CFBundleShortVersionString raw -o - "$plist")" = "$version" \
    && "$(plutil -extract CFBundleExecutable raw -o - "$plist")" = ShiftAnything \
    && "$(lipo -archs "$app/Contents/MacOS/ShiftAnything")" = arm64 ]] || fail 'Unexpected macOS bundle identity, version or architecture.'
  check_bundle_modes "$app"
  codesign --verify --strict --deep --all-architectures "$app"
  verify_signature "$app" "$app_id"
  xcrun stapler validate "$app" >/dev/null
  assess --type execute "$app"
}

verify_signature "$dmg" "$app_id.disk-image"
xcrun stapler validate "$dmg" >/dev/null
assess --type open --context context:primary-signature "$dmg"
hdiutil verify -quiet "$dmg"
hdiutil attach "$dmg" -readonly -nobrowse -noautoopen -mountpoint "$mount" -quiet
mounted=1
verify_app "$mount/$app_name"
/usr/bin/tar --mac-metadata -czf "$assets/$mac_name" -C "$mount" "$app_name"
/usr/bin/tar --mac-metadata -xzpf "$assets/$mac_name" -C "$work/extracted"
verify_app "$work/extracted/$app_name"
hdiutil detach "$mount" -quiet
mounted=0
printf 'PASS notarized macOS installer and updater archive round-trip\n'

[[ "$(/usr/bin/ar -p "$deb" debian-binary)" = 2.0 ]] || fail 'Invalid Debian package format.'
members="$(/usr/bin/ar -t "$deb")"
control="$(grep -E '^control\.tar(\.(gz|xz|zst|bz2))?$' <<< "$members")" || fail 'Missing Debian control archive.'
data="$(grep -E '^data\.tar(\.(gz|xz|zst|bz2))?$' <<< "$members")" || fail 'Missing Debian data archive.'
[[ "$control" != *$'\n'* && "$data" != *$'\n'* ]] || fail 'Unexpected Debian archive layout.'
/usr/bin/ar -p "$deb" "$control" | /usr/bin/tar -xOf - ./control > "$work/deb-control"
grep -qx 'Package: shiftanything' "$work/deb-control"
grep -qx 'Architecture: amd64' "$work/deb-control"
grep -Fxq "Version: $version" "$work/deb-control"
/usr/bin/ar -p "$deb" "$data" | /usr/bin/tar -xOf - ./usr/bin/shiftanything > "$work/linux-launcher"
/usr/bin/file -b "$work/linux-launcher" | grep -Eq '^ELF 64-bit .*x86-64'
printf 'PASS Linux package identity, release checksum and executable architecture\n'

sign() {
  local asset=$1
  "${cli[@]}" signer sign --app-version "$version" "$asset" </dev/null >/dev/null 2>&1 \
    || fail "Tauri signer could not sign ${asset##*/} with the configured key path."
  [[ -s "$asset.sig" ]] || fail "Missing Tauri signature for ${asset##*/}."
}
sign "$assets/$mac_name"
sign "$deb"
python3 - "$assets" "$repo" "$version" "$mac_name" "$linux_name" <<'PY'
import base64
import json
import os
from pathlib import Path
import sys

assets = Path(sys.argv[1])
repo, version, mac_name, linux_name = sys.argv[2:]
public = os.environ.get("SHIFT_LAUNCHER_UPDATE_PUBKEY")
public_record = None
if public:
    lines = base64.b64decode(public.strip(), validate=True).decode("utf-8").splitlines()
    if len(lines) != 2:
        sys.exit("Unexpected Tauri public key format.")
    public_record = base64.b64decode(lines[1], validate=True)
    if len(public_record) != 42:
        sys.exit("Unexpected Tauri public key record.")
platforms = {}
key_ids = set()
for target, name in [("darwin-aarch64", mac_name), ("linux-x86_64-deb", linux_name)]:
    signature = (assets / f"{name}.sig").read_text().strip()
    lines = base64.b64decode(signature, validate=True).decode("utf-8").splitlines()
    if len(lines) != 4 or not lines[2].startswith("trusted comment: ") or f"version:{version}" not in lines[2].split("\t"):
        sys.exit(f"The Tauri signature for {target} does not bind the requested version.")
    record = base64.b64decode(lines[1], validate=True)
    if len(record) != 74:
        sys.exit(f"Unexpected Tauri signature record for {target}.")
    key_id = record[2:10]
    key_ids.add(key_id)
    if public_record is not None and key_id != public_record[2:10]:
        sys.exit(f"The Tauri signing key for {target} does not match the configured updater public key.")
    platforms[target] = {"signature": signature, "url": f"https://github.com/{repo}/releases/download/v{version}/{name}"}
if len(key_ids) != 1:
    sys.exit("The updater assets were signed by different keys.")
(assets / "latest.json").write_text(json.dumps({"version": version, "platforms": platforms}, indent=2) + "\n")
PY
(
  cd "$assets"
  shasum -a 256 "$mac_name" "$mac_name.sig" "$linux_name" "$linux_name.sig" latest.json > SHA256SUMS.txt
  shasum -a 256 -c SHA256SUMS.txt
)
# A fresh directory reserves the destination. The manifest is exposed only once its assets exist.
mkdir -- "$output"
for file in "$mac_name" "$mac_name.sig" "$linux_name" "$linux_name.sig" SHA256SUMS.txt latest.json; do
  mv -- "$assets/$file" "$output/$file"
done
printf 'PASS local updater assets ready for GitHub release v%s in %s\n' "$version" "$output"
