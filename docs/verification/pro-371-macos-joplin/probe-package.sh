#!/bin/bash
set -euo pipefail
proof=$(cd "$(dirname "$0")" && pwd -P)
[[ "$proof" == /private/tmp/shift-joplin-native.* ]]
tag=${1:?version}
case "$tag" in
  v3.7.22|v3.7.24) ;;
  *) exit 1;;
esac
version=${tag#v}
receipt="$proof/$tag.json"
asset="Joplin-$version-arm64.DMG"
scratch=$(mktemp -d "$proof/payload-$tag.XXXXXXXX")
mount="$scratch/mount"
mkdir "$mount"
touch "$scratch/.shift-proof"
mounted=0
cleanup() {
  result=$?
  if (( mounted )); then
    /usr/bin/hdiutil detach "$mount" -quiet || return
  fi
  if [[ "$scratch" == "$proof"/payload-* && -f "$scratch/.shift-proof" ]]; then
    /bin/rm -rf -- "$scratch"
  fi
  printf 'CLEANUP %s status=%s\n' "$tag" "$result"
}
trap cleanup EXIT
read -r size digest url < <(/usr/bin/python3 - "$receipt" "$asset" <<'PY'
import json, sys
with open(sys.argv[1]) as f:
    release = json.load(f)
assert not release['prerelease'] and not release['draft']
asset = next(a for a in release['assets'] if a['name'] == sys.argv[2])
assert asset['digest'].startswith('sha256:')
print(asset['size'], asset['digest'].split(':', 1)[1], asset['browser_download_url'])
PY
)
echo "PACKAGE $tag $url $size $digest"
dmg="$scratch/$asset"
/usr/bin/curl --proto '=https' --proto-redir '=https' -fsSL --retry 1 --max-time 120 "$url" -o "$dmg"
[[ $(/usr/bin/stat -f '%z' "$dmg") == "$size" ]]
printf '%s  %s\n' "$digest" "$dmg" | /usr/bin/shasum -a 256 -c -
/usr/bin/hdiutil attach -nobrowse -noautoopen -readonly -noverify -mountpoint "$mount" "$dmg"
mounted=1
bundle="$mount/Joplin.app"
info="$bundle/Contents/Info.plist"
for key in CFBundleIdentifier CFBundleShortVersionString CFBundleExecutable LSMinimumSystemVersion; do
  printf '%s ' "$key"
  /usr/libexec/PlistBuddy -c "Print :$key" "$info"
done
executable=$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$info")
/usr/bin/lipo -archs "$bundle/Contents/MacOS/$executable"
/usr/bin/vtool -show-build "$bundle/Contents/MacOS/$executable"
/usr/bin/codesign -dv --verbose=4 "$bundle" 2>&1 | /usr/bin/grep -E 'Identifier=|Authority=|TeamIdentifier='
/usr/bin/codesign --verify --strict --deep "$bundle"
/usr/bin/codesign --verify --check-notarization -R=notarized "$bundle"
/usr/sbin/spctl --assess --type execute -vv "$bundle"
echo "PASS $tag official hash, strict signature, notarization, Gatekeeper"
