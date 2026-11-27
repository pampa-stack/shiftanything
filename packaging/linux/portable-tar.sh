#!/usr/bin/env bash
# Build a portable archive from the verified Linux release DEB.
set -euo pipefail
export LC_ALL=C

fail() { printf 'error: %s\n' "$*" >&2; exit 1; }
[[ $# = 4 ]] || fail 'Usage: portable-tar.sh VERSION DEB DEB_SHA256 OUTPUT_DIRECTORY'
version=$1
deb=$2
expected_sha=$3
output=$4
[[ "$version" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]] || fail 'Expected a three-part stable version.'
[[ "$expected_sha" =~ ^[a-f0-9]{64}$ && -f "$deb" && -d "$output" ]] || fail 'Expected a package, its SHA-256, and an existing output directory.'
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
work="$(mktemp -d "$output/.shiftanything-portable.XXXXXXXX")"
trap 'rm -rf -- "$work"' EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
cp -- "$deb" "$work/source.deb"
deb="$work/source.deb"
[[ "$(shasum -a 256 < "$deb" | awk '{print $1}')" = "$expected_sha" ]] || fail 'The Linux package does not match its release receipt.'
[[ "$(ar -p "$deb" debian-binary)" = 2.0 ]] || fail 'Invalid Debian package format.'
members="$(ar -t "$deb")"
control="$(grep -E '^control\.tar(\.(gz|xz|zst|bz2))?$' <<< "$members")" || fail 'Missing control archive.'
data="$(grep -E '^data\.tar(\.(gz|xz|zst|bz2))?$' <<< "$members")" || fail 'Missing data archive.'
[[ "$control" != *$'\n'* && "$data" != *$'\n'* ]] || fail 'Unexpected Debian archive layout.'
if [[ "$control" = *.zst || "$data" = *.zst ]]; then
  command -v zstd >/dev/null || fail 'Install zstd to read this Debian package.'
fi
ar -p "$deb" "$control" > "$work/$control"
ar -p "$deb" "$data" > "$work/$data"

extract() {
  tar -xOf "$work/$1" "./$2"
}
extract "$control" control > "$work/control"
grep -Fxq 'Package: shiftanything' "$work/control"
grep -Fxq "Version: $version" "$work/control"
grep -Fxq 'Architecture: amd64' "$work/control"
extract "$data" usr/bin/shiftanything > "$work/shiftanything"
file -b "$work/shiftanything" | grep -Eq '^ELF 64-bit .*x86-64'
extract "$data" usr/share/icons/hicolor/256x256/apps/it.promptica.shiftanything.png > "$work/icon.png"
for legal in LICENSE-MIT LICENSE-APACHE NOTICE ATTRIBUTION.md; do
  extract "$data" "usr/share/doc/shiftanything/$legal" > "$work/$legal"
done
cp -- "$here/portable-README.md" "$work/README.md"

name="ShiftAnything-$version-linux-amd64"
python3 - "$work" "$name" <<'PY'
import gzip
import os
from pathlib import Path
import sys
import tarfile

work, name = Path(sys.argv[1]), sys.argv[2]
path = work / f"{name}.tar.gz"
with (
    path.open("xb") as raw,
    gzip.GzipFile(fileobj=raw, mode="wb", mtime=0, filename="") as compressed,
    tarfile.open(fileobj=compressed, mode="w|") as archive,
):
    directory = tarfile.TarInfo(name)
    directory.type = tarfile.DIRTYPE
    directory.mode = 0o755
    archive.addfile(directory)
    for filename in ("shiftanything", "icon.png", "README.md", "LICENSE-MIT", "LICENSE-APACHE", "NOTICE", "ATTRIBUTION.md"):
        source = work / filename
        entry = tarfile.TarInfo(f"{name}/{filename}")
        entry.size = source.stat().st_size
        entry.mode = 0o755 if filename == "shiftanything" else 0o644
        with source.open("rb") as content:
            archive.addfile(entry, content)

os.link(path, work.parent / path.name)
PY
printf 'Portable Linux archive: %s/%s.tar.gz\n' "$output" "$name"
