#!/usr/bin/env bash
# Assemble an isolated, ad-hoc signed test bundle. Never installs or launches it.
# Usage: local-app.sh NEW_OUTPUT_DIRECTORY
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
[[ $# = 1 ]] || { echo 'supply a new output directory' >&2; exit 2; }
OUTPUT="$1"
mkdir "$OUTPUT"
BINARY="${CARGO_TARGET_DIR:-$ROOT/target}/debug/artcraft-launcher"
[[ -x "$BINARY" ]] || { echo 'build the desktop binary first' >&2; exit 1; }
APP="$OUTPUT/Sh*ftAnything.app"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources" "$OUTPUT/icon.iconset"
cp "$BINARY" "$APP/Contents/MacOS/ShiftAnything"
for file in LICENSE-MIT LICENSE-APACHE NOTICE ATTRIBUTION.md; do
  cp "$ROOT/$file" "$APP/Contents/Resources/"
done
for size in 16 32 64 128 256 512; do
  sips -z "$size" "$size" "$ROOT/assets/shiftanything/icon.png" --out "$OUTPUT/icon.iconset/icon_${size}x${size}.png" >/dev/null
done
# iconutil's 2x entries carry the same pixels under the standard Retina names.
for size in 16 32 128 256; do
  double=$((size * 2))
  cp "$OUTPUT/icon.iconset/icon_${double}x${double}.png" "$OUTPUT/icon.iconset/icon_${size}x${size}@2x.png"
done
iconutil -c icns "$OUTPUT/icon.iconset" -o "$APP/Contents/Resources/ShiftAnything.icns"
VERSION="$(awk '/^\[/{active=($0=="[workspace.package]")} active && $1=="version" {gsub(/"/,"",$3);print $3;exit}' "$ROOT/Cargo.toml")"
cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0"><dict>
<key>CFBundleIdentifier</key><string>it.promptica.shiftanything</string>
<key>CFBundleName</key><string>Sh*ftAnything</string>
<key>CFBundleDisplayName</key><string>Sh*ftAnything</string>
<key>CFBundleExecutable</key><string>ShiftAnything</string>
<key>CFBundleIconFile</key><string>ShiftAnything</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>$VERSION</string>
<key>CFBundleVersion</key><string>$VERSION</string>
<key>LSMinimumSystemVersion</key><string>11.0</string>
<key>NSHighResolutionCapable</key><true/>
<key>NSHumanReadableCopyright</key><string>Promptica; ArtCraft contributors. MIT OR Apache-2.0. See bundled notices.</string>
</dict></plist>
PLIST
plutil -lint "$APP/Contents/Info.plist"
codesign --force --sign - --timestamp=none "$APP"
codesign --verify --deep --strict "$APP"
"$APP/Contents/MacOS/ShiftAnything" --version
printf 'PASS local bundle: %s\nAd-hoc test only; not notarized or installed.\n' "$APP"
