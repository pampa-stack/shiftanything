#!/bin/bash
set -euo pipefail
proof=$(cd "$(dirname "$0")" && pwd -P)
[[ "$proof" == /private/tmp/shift-macos-daily.* ]]
scratch=$(/usr/bin/mktemp -d /private/tmp/shift-lo-native-scratch.XXXXXXXX)
home="$scratch/home"
cleanup() {
  result=$?
  if [[ "$scratch" == /private/tmp/shift-lo-native-scratch.* && -f "$home/.shift-lo-native-proof" ]]; then
    /bin/rm -rf -- "$scratch"
  fi
  printf 'CLEANUP %s exit=%s\n' "$scratch" "$result"
}
trap cleanup EXIT
/bin/mkdir -p "$home/Applications" "$home/.Trash" "$home/Library/Application Support/LibreOffice" "$scratch/tmp"
/usr/bin/touch "$home/.shift-lo-native-proof"
marker="$home/Library/Application Support/LibreOffice/fixture"
printf 'ShiftAnything LibreOffice native lifecycle profile marker\n' > "$marker"
checksum=$(/usr/bin/shasum -a 256 "$marker")
rule='anchor apple generic and certificate leaf[subject.OU] = "7P5S3ZLCN7" and identifier "org.libreoffice.script"'
for phase in install update remove; do
  /usr/bin/env -i HOME="$home" CFFIXED_USER_HOME="$home" TMPDIR="$scratch/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$proof/libreoffice-native-cycle" "$phase"
  [[ $(/usr/bin/shasum -a 256 "$marker") == "$checksum" ]]
  bundle="$home/Applications/LibreOffice.app"
  version=26.8.3.1
  if [[ "$phase" == "remove" ]]; then
    [[ ! -e "$bundle" ]]
    bundle="$home/.Trash/LibreOffice.app"
  elif [[ "$phase" == "install" ]]; then
    version=26.8.2.2
  fi
  [[ $(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$bundle/Contents/Info.plist") == "$version" ]]
  [[ $(/usr/bin/lipo -archs "$bundle/Contents/MacOS/soffice") == arm64 ]]
  /usr/bin/codesign --verify --strict --deep "-R=$rule" "$bundle"
  /usr/sbin/spctl --assess --type execute -vv "$bundle"
  echo "PASS SIGNATURE + PROFILE MARKER $phase $version"
done
