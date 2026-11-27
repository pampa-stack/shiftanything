#!/bin/bash
set -euo pipefail
proof=$(cd "$(dirname "$0")" && pwd -P)
[[ "$proof" == /private/tmp/shift-joplin-native.* ]]
scratch=$(/usr/bin/mktemp -d /private/tmp/shift-joplin-scratch.XXXXXXXX)
home="$scratch/home"
cleanup() {
  result=$?
  if [[ "$scratch" == /private/tmp/shift-joplin-scratch.* && -f "$home/.shift-joplin-proof" ]]; then
    /bin/rm -rf -- "$scratch"
  fi
  printf 'CLEANUP %s exit=%s\n' "$scratch" "$result"
}
trap cleanup EXIT
/bin/mkdir -p "$home/Applications" "$home/.Trash" "$home/Library/Application Support/Joplin" "$scratch/tmp"
/usr/bin/touch "$home/.shift-joplin-proof"
marker="$home/Library/Application Support/Joplin/fixture"
printf 'ShiftAnything Joplin native lifecycle profile marker\n' > "$marker"
checksum=$(/usr/bin/shasum -a 256 "$marker")
rule='anchor apple generic and certificate leaf[subject.OU] = "A9BXAFS6CT" and identifier "net.cozic.joplin-desktop"'
for phase in install update remove; do
  /usr/bin/env -i HOME="$home" CFFIXED_USER_HOME="$home" TMPDIR="$scratch/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$proof/joplin-native-cycle" "$phase"
  [[ $(/usr/bin/shasum -a 256 "$marker") == "$checksum" ]]
  bundle="$home/Applications/Joplin.app"
  version=3.7.24
  if [[ "$phase" == "remove" ]]; then
    [[ ! -e "$bundle" ]]
    bundle="$home/.Trash/Joplin.app"
  elif [[ "$phase" == "install" ]]; then
    version=3.7.22
  fi
  [[ $(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$bundle/Contents/Info.plist") == "$version" ]]
  [[ $(/usr/bin/lipo -archs "$bundle/Contents/MacOS/Joplin") == arm64 ]]
  /usr/bin/codesign --verify --strict --deep "$bundle"
  /usr/bin/codesign --verify --strict "-R=$rule" "$bundle"
  /usr/sbin/spctl --assess --type execute -vv "$bundle"
  echo "PASS SIGNATURE + PROFILE MARKER $phase $version"
done
