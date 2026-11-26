#!/usr/bin/env bash
# Build an ad-hoc local Sh*ftAnything test DMG from the existing debug binary.
# Usage: local-dmg.sh NEW_OUTPUT_DIRECTORY. Never installs, opens GUI or publishes.
set -euo pipefail
umask 077
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
[[ $# = 1 ]] || { echo 'supply a new output directory' >&2; exit 2; }
OUTPUT="$1"
# local-app.sh creates OUTPUT exclusively and refuses to overwrite any existing folder.
"$ROOT/packaging/macos/local-app.sh" "$OUTPUT"
APP="$OUTPUT/Sh*ftAnything.app"
EXE="$APP/Contents/MacOS/ShiftAnything"
VERSION="$(plutil -extract CFBundleShortVersionString raw -o - "$APP/Contents/Info.plist")"
ARCH="$(uname -m)"
lipo "$EXE" -verify_arch "$ARCH"
DMG="$OUTPUT/ShiftAnything-$VERSION-macos-$ARCH-local.dmg"
STAGE="$OUTPUT/stage"
MOUNT="$OUTPUT/mount"
mkdir "$STAGE" "$MOUNT"
ditto "$APP" "$STAGE/Sh*ftAnything.app"
ln -s /Applications "$STAGE/Applications"
hdiutil create -srcfolder "$STAGE" -volname 'Sh*ftAnything' -fs HFS+ -format UDZO "$DMG"
codesign --force --sign - --timestamp=none "$DMG"
codesign --verify --strict "$DMG"
hdiutil verify "$DMG"
MOUNTED=0
cleanup() {
  if [[ "$MOUNTED" = 1 ]]; then hdiutil detach "$MOUNT" -quiet; fi
}
trap cleanup EXIT
hdiutil attach "$DMG" -readonly -nobrowse -noautoopen -mountpoint "$MOUNT" -quiet
MOUNTED=1
BUNDLE="$MOUNT/Sh*ftAnything.app"
[[ "$(plutil -extract CFBundleIdentifier raw -o - "$BUNDLE/Contents/Info.plist")" = it.promptica.shiftanything ]]
[[ "$(plutil -extract CFBundleDisplayName raw -o - "$BUNDLE/Contents/Info.plist")" = 'Sh*ftAnything' ]]
[[ "$(plutil -extract CFBundleExecutable raw -o - "$BUNDLE/Contents/Info.plist")" = ShiftAnything ]]
[[ "$(plutil -extract CFBundleShortVersionString raw -o - "$BUNDLE/Contents/Info.plist")" = "$VERSION" ]]
codesign --verify --deep --strict "$BUNDLE"
lipo "$BUNDLE/Contents/MacOS/ShiftAnything" -verify_arch "$ARCH"
for file in LICENSE-MIT LICENSE-APACHE NOTICE ATTRIBUTION.md; do
  cmp "$ROOT/$file" "$BUNDLE/Contents/Resources/$file"
done
ditto "$BUNDLE" "$OUTPUT/verified-copy/Sh*ftAnything.app"
codesign --verify --deep --strict "$OUTPUT/verified-copy/Sh*ftAnything.app"
"$OUTPUT/verified-copy/Sh*ftAnything.app/Contents/MacOS/ShiftAnything" --version
hdiutil detach "$MOUNT" -quiet
MOUNTED=0
shasum -a 256 "$DMG" > "$OUTPUT/SHA256SUMS.txt"
printf 'PASS local DMG: %s\nAd-hoc test only; not notarized, installed, launched as GUI or published.\n' "$DMG"
