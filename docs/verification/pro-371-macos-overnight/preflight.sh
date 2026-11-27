#!/bin/bash
set -euo pipefail
umask 077
cd "$(dirname "$0")"
root="$(pwd -P)"
case "$root" in /private/tmp/shift-air-overnight.*) ;; *) exit 2 ;; esac
test -f .shift-owned
test "$(uname -m)" = arm64
id="$1"
tag="$2"
asset="$3"
url="$4"
bytes="${5:-}"
digest="${6:-}"
[[ "$id" =~ ^[a-z0-9-]+$ ]]
[[ "$tag" =~ ^v?[0-9]+\.[0-9]+(\.[0-9]+)?$ ]]
[[ "$asset" =~ ^[A-Za-z0-9._-]+\.dmg$ ]]
[[ "$url" == https://*"/$asset" ]]
work="$root/preflight/$id/$tag"
mkdir -p "$work/mount"
dmg="$work/$asset"
mnt="$work/mount"
mounted=0
trap 'if (( mounted )); then hdiutil detach "$mnt" >/dev/null 2>&1 || true; fi; /bin/rm -f -- "$dmg"' EXIT
date +%FT%T%z
df -k "$work" | awk 'NR==2{print; exit !($4>=8388608)}'
curl --proto '=https' --proto-redir '=https' --fail --silent --show-error --location --max-time 300 --max-filesize "${bytes:-800000000}" --output "$dmg" "$url"
actual_bytes="$(stat -f %z "$dmg")"
actual_hash="$(shasum -a 256 "$dmg" | awk '{print $1}')"
[[ -z "$bytes" || "$actual_bytes" == "$bytes" ]]
[[ -z "$digest" || "$actual_hash" == "$digest" ]]
printf 'ARTIFACT %s %s %s\n' "$actual_hash" "$actual_bytes" "$url"
hdiutil attach -readonly -nobrowse -noautoopen -verify -mountpoint "$mnt" "$dmg" >/dev/null
mounted=1
apps=( "$mnt/"*.app )
test "${#apps[@]}" = 1
app="${apps[0]}"
test -d "$app" && test ! -L "$app"
printf 'BUNDLE=%s\n' "${app##*/}"
plist="$app/Contents/Info.plist"
for key in CFBundleIdentifier CFBundleShortVersionString CFBundleVersion CFBundleExecutable LSMinimumSystemVersion; do
    value=$(/usr/bin/plutil -extract "$key" raw "$plist" 2>/dev/null || true)
    printf '%s=%s\n' "$key" "$value"
done
binary=$(/usr/bin/plutil -extract CFBundleExecutable raw "$plist")
[[ "$binary" =~ ^[A-Za-z0-9._\ -]+$ ]]
/usr/bin/file "$app/Contents/MacOS/$binary"
/usr/bin/lipo "$app/Contents/MacOS/$binary" -verify_arch arm64
/usr/bin/vtool -arch arm64 -show-build "$app/Contents/MacOS/$binary"
/usr/bin/codesign --display --verbose=4 "$app" 2>&1 | /usr/bin/grep -E '^(Authority|TeamIdentifier|Identifier|Format|Timestamp)='
/usr/bin/codesign --verify --all-architectures --deep --strict "$app"
/usr/bin/codesign --verify --check-notarization -R=notarized "$app"
verdict=$(/usr/sbin/spctl --assess --ignore-cache --no-cache --type execute --verbose=4 "$app" 2>&1)
printf '%s\n' "$verdict"
[[ "$verdict" == *'source=Notarized Developer ID'* ]]
hdiutil detach "$mnt" >/dev/null
mounted=0
printf 'PASS PREFLIGHT %s %s\n' "$id" "$tag"
