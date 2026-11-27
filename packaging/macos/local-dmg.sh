#!/usr/bin/env bash
# Build an ad-hoc local Sh*ftAnything test DMG from the existing debug binary.
# Usage: local-dmg.sh [--tauri] NEW_OUTPUT_DIRECTORY. Never installs, opens GUI or publishes.
set -euo pipefail
umask 077
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=check-bundle.sh
. "$ROOT/packaging/macos/check-bundle.sh"
LAYOUT="$ROOT/packaging/macos/dmg/shiftanything"
SUFFIX=local
case "$#:${1:-}" in
  1:*) OUTPUT="$1" ;;
  2:--tauri) OUTPUT="$2"; SUFFIX=tauri-local ;;
  *) echo 'usage: local-dmg.sh [--tauri] NEW_OUTPUT_DIRECTORY' >&2; exit 2 ;;
esac
# local-app.sh creates OUTPUT exclusively and refuses to overwrite any existing folder.
"$ROOT/packaging/macos/local-app.sh" "$@"
APP="$OUTPUT/Sh*ftAnything.app"
EXE="$APP/Contents/MacOS/ShiftAnything"
VERSION="$(plutil -extract CFBundleShortVersionString raw -o - "$APP/Contents/Info.plist")"
ARCH="$(uname -m)"
lipo "$EXE" -verify_arch "$ARCH"
DMG="$OUTPUT/ShiftAnything-$VERSION-macos-$ARCH-$SUFFIX.dmg"
STAGE="$OUTPUT/stage"
mkdir -m 755 "$STAGE"
ditto "$APP" "$STAGE/Sh*ftAnything.app"
stage_dmg_layout "$STAGE" "$LAYOUT"
hdiutil create -srcfolder "$STAGE" -volname 'Sh*ftAnything' -fs HFS+ -format UDZO "$DMG"
codesign --force --sign - --timestamp=none "$DMG"
codesign --verify --strict "$DMG"
# The image can live on an external drive. DiskImages mounts it on the system volume.
hdiutil verify "$DMG"
MOUNT="$(mktemp -d /private/tmp/shiftanything-local-dmg.XXXXXXXX)"
MOUNTED=0
cleanup() {
  if [[ "$MOUNTED" = 1 ]]; then hdiutil detach "$MOUNT" -quiet || return; fi
  rmdir "$MOUNT"
}
trap cleanup EXIT
hdiutil attach "$DMG" -readonly -nobrowse -noautoopen -mountpoint "$MOUNT"
MOUNTED=1
check_dmg_layout "$MOUNT" "$LAYOUT"
BUNDLE="$MOUNT/Sh*ftAnything.app"
check_bundle_modes "$BUNDLE"
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
check_bundle_modes "$OUTPUT/verified-copy/Sh*ftAnything.app"
codesign --verify --deep --strict "$OUTPUT/verified-copy/Sh*ftAnything.app"
check_bundle_version "$OUTPUT/verified-copy/Sh*ftAnything.app" "$VERSION"
hdiutil detach "$MOUNT" -quiet
MOUNTED=0
shasum -a 256 "$DMG" > "$OUTPUT/SHA256SUMS.txt"
printf 'PASS local DMG: %s\nAd-hoc test only; not notarized, installed, launched as GUI or published.\n' "$DMG"
