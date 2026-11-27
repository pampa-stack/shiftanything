#!/bin/bash
set -euo pipefail

probe=${1:?Pass the path to the linked native probe}
[[ "$(uname -sm)" == "Darwin arm64" ]]
scratch=$(/usr/bin/mktemp -d /private/tmp/shift-zotero-native-scratch.XXXXXXXX)
home="$scratch/home"
cleanup() {
  status=$?
  if [[ "$scratch" == /private/tmp/shift-zotero-native-scratch.* && -f "$home/.shift-zotero-native-proof" ]]; then
    /bin/rm -rf -- "$scratch"
  fi
  printf 'CLEANUP %s exit=%s\n' "$scratch" "$status"
}
trap cleanup EXIT
/bin/mkdir -p "$home/Applications" "$home/.Trash" "$home/Zotero" "$scratch/tmp"
/usr/bin/touch "$home/.shift-zotero-native-proof"
marker="$home/Zotero/fixture"
printf 'ShiftAnything Zotero native lifecycle profile marker\n' > "$marker"
checksum=$(/usr/bin/shasum -a 256 "$marker")
requirement='anchor apple generic and certificate leaf[subject.OU] = "8LAYR367YV" and identifier "org.zotero.zotero"'
for phase in install update remove; do
  /usr/bin/env -i HOME="$home" CFFIXED_USER_HOME="$home" TMPDIR="$scratch/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$probe" "$phase"
  [[ $(/usr/bin/shasum -a 256 "$marker") == "$checksum" ]]
  bundle="$home/Applications/Zotero.app"
  version=10.0.9
  if [[ "$phase" == remove ]]; then
    [[ ! -e "$bundle" ]]
    bundle="$home/.Trash/Zotero.app"
  elif [[ "$phase" == install ]]; then
    version=10.0.8
  fi
  [[ $(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$bundle/Contents/Info.plist") == "$version" ]]
  executable=$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$bundle/Contents/Info.plist")
  /usr/bin/lipo "$bundle/Contents/MacOS/$executable" -verify_arch arm64
  /usr/bin/codesign --verify --strict --deep "-R=$requirement" "$bundle"
  /usr/sbin/spctl --assess --type execute -vv "$bundle"
  printf 'PASS SIGNATURE + PROFILE MARKER %s %s\n' "$phase" "$version"
done
