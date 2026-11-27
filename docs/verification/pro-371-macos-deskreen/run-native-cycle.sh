#!/bin/bash
set -euo pipefail

probe=${1:?Provide the compiled native probe}
[[ "$(/usr/bin/uname -sm)" == "Darwin arm64" ]]
scratch=$(/usr/bin/mktemp -d /private/tmp/shift-deskreen-native.XXXXXXXX)
home="$scratch/home"
trap '/bin/rm -rf -- "$scratch"' EXIT
/bin/mkdir -p "$home/Applications" "$home/.Trash" "$home/Library/Application Support/Deskreen CE" "$scratch/tmp"
/usr/bin/touch "$home/.deskreen-verification-profile"
marker="$home/Library/Application Support/Deskreen CE/fixture"
printf 'Deskreen proof, synthetic user data\n' > "$marker"
checksum=$(/usr/bin/shasum -a 256 "$marker")
requirement='anchor apple generic and certificate leaf[subject.OU] = "3MGY23CG64" and identifier "com.deskreen-ce.app"'
for phase in install update remove; do
  /usr/bin/env -i HOME="$home" CFFIXED_USER_HOME="$home" TMPDIR="$scratch/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$probe" "$phase"
  [[ "$(/usr/bin/shasum -a 256 "$marker")" == "$checksum" ]]
  bundle="$home/Applications/Deskreen CE.app"
  version=3.2.16
  if [[ "$phase" == remove ]]; then
    [[ ! -e "$bundle" ]]
    bundle="$home/.Trash/Deskreen CE.app"
  elif [[ "$phase" == install ]]; then
    version=3.2.15
  fi
  plist="$bundle/Contents/Info.plist"
  [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$plist")" == "$version" ]]
  executable=$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$plist")
  /usr/bin/lipo "$bundle/Contents/MacOS/$executable" -verify_arch arm64
  minimum=$(/usr/libexec/PlistBuddy -c 'Print :LSMinimumSystemVersion' "$plist" 2>/dev/null || true)
  if [[ -z "$minimum" ]]; then
    minimum=$(/usr/bin/vtool -show-build "$bundle/Contents/MacOS/$executable" | /usr/bin/awk '$1 == "minos" { print $2; exit }')
  fi
  [[ -n "$minimum" ]]
  /usr/bin/codesign --verify --deep --strict "-R=$requirement" "$bundle"
  /usr/sbin/spctl --assess --type execute -vv "$bundle"
  printf 'PASS publisher, version %s, arm64, macOS minimum %s and profile marker\n' "$version" "$minimum"
done
