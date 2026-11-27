#!/usr/bin/env bash
# Package a local Ubuntu 24.04 amd64 build without installing it.
set -euo pipefail
umask 077
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
[[ $# = 1 ]] || { echo 'usage: local-deb.sh NEW_OUTPUT_DIRECTORY' >&2; exit 2; }
[[ "$(dpkg --print-architecture)" = amd64 ]] || { echo 'Build on amd64 Ubuntu 24.04' >&2; exit 1; }
OUTPUT="$1"
BINARY="${CARGO_TARGET_DIR:-$ROOT/target}/debug/shiftanything-tauri-proof"
[[ -x "$BINARY" ]] || { echo 'Build shiftanything-tauri-proof first' >&2; exit 1; }
VERSION="$(awk '/^\[/{active=($0=="[workspace.package]")} active && $1=="version" {gsub(/"/,"",$3);print $3;exit}' "$ROOT/Cargo.toml")"
[[ "$("$BINARY" --version)" = "shiftanything $VERSION" ]]

mkdir -- "$OUTPUT"
OUTPUT="$(cd "$OUTPUT" && pwd)"
STAGE="$OUTPUT/stage"
APP_ID=it.promptica.shiftanything
mkdir -m 0755 "$STAGE"
install -d -m 0755 "$STAGE/DEBIAN"
install -D -m 0755 "$BINARY" "$STAGE/usr/bin/shiftanything"
install -D -m 0644 "$ROOT/apps/shiftanything-tauri-proof/icons/icon.png" "$STAGE/usr/share/icons/hicolor/256x256/apps/$APP_ID.png"
for file in LICENSE-MIT LICENSE-APACHE NOTICE ATTRIBUTION.md; do
  install -D -m 0644 "$ROOT/$file" "$STAGE/usr/share/doc/shiftanything/$file"
done
install -d -m 0755 "$STAGE/usr/share/applications"
DESKTOP="$STAGE/usr/share/applications/$APP_ID.desktop"
cat > "$DESKTOP" <<DESKTOP
[Desktop Entry]
Type=Application
Name=Sh*ftAnything
Comment=Find and manage open-source apps
Comment[it]=Trova e gestisci le app open source
Exec=shiftanything
TryExec=shiftanything
Icon=$APP_ID
Terminal=false
Categories=System;PackageManager;
DESKTOP
chmod 0644 "$DESKTOP"
if command -v desktop-file-validate >/dev/null; then
  desktop-file-validate "$DESKTOP"
fi
cat > "$STAGE/DEBIAN/control" <<CONTROL
Package: shiftanything
Version: $VERSION
Architecture: amd64
Section: utils
Priority: optional
Maintainer: Promptica <info@promptica.it>
Depends: libc6 (>= 2.39), libcairo2, libgcc-s1, libgdk-pixbuf-2.0-0, libglib2.0-0t64, libgtk-3-0t64, libjavascriptcoregtk-4.1-0, libsoup-3.0-0, libwebkit2gtk-4.1-0
Recommends: flatpak, pkexec
Description: Desktop launcher for open-source apps
 Browse, compare, install and update supported applications from Sh*ftAnything.
CONTROL
chmod 0644 "$STAGE/DEBIAN/control"

DEB="$OUTPUT/shiftanything_${VERSION}_amd64-local.deb"
dpkg-deb --root-owner-group --build "$STAGE" "$DEB"
dpkg-deb --raw-extract "$DEB" "$OUTPUT/verified"
[[ "$(dpkg-deb -f "$DEB" Package)" = shiftanything ]]
[[ "$(dpkg-deb -f "$DEB" Version)" = "$VERSION" ]]
[[ "$(dpkg-deb -f "$DEB" Architecture)" = amd64 ]]
cmp "$DESKTOP" "$OUTPUT/verified/usr/share/applications/$APP_ID.desktop"
cmp "$STAGE/usr/share/icons/hicolor/256x256/apps/$APP_ID.png" "$OUTPUT/verified/usr/share/icons/hicolor/256x256/apps/$APP_ID.png"
cmp "$BINARY" "$OUTPUT/verified/usr/bin/shiftanything"
"$OUTPUT/verified/usr/bin/shiftanything" --version
(cd "$OUTPUT" && sha256sum "${DEB##*/}" > SHA256SUMS.txt)
printf 'PASS local Ubuntu amd64 package: %s\n' "$DEB"
