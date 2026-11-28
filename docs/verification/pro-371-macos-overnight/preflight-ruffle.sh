#!/bin/bash
set -euo pipefail
umask 077
cd "$(dirname "$0")"
[[ $PWD == /private/tmp/shift-air-overnight.* && -f .shift-owned && $(uname -m) == arm64 ]]
tag=$1 expected_bytes=$2 expected_hash=$3
[[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ && "$expected_hash" =~ ^[0-9a-f]{64}$ ]]
root="ruffle-$tag"
mkdir "$root"
cd "$root"
file="ruffle-${tag#v}-macos-universal.tar.gz"
url="https://github.com/ruffle-rs/ruffle/releases/download/$tag/$file"
date +%FT%T%z
curl --proto '=https' --proto-redir '=https' --fail --silent --show-error --location --max-filesize "$expected_bytes" --max-time 120 --output "$file" "$url"
[[ $(stat -f %z "$file") == "$expected_bytes" ]]
printf '%s  %s\n' "$expected_hash" "$file" | /usr/bin/shasum -a 256 -c -
printf 'ARTIFACT %s %s %s\n' "$expected_hash" "$expected_bytes" "$url"
mkdir payload
python3 - "$file" <<'PY'
import sys, tarfile
with tarfile.open(sys.argv[1], 'r:gz') as archive:
    members = archive.getmembers()
    if len(members) > 2000 or sum(m.size for m in members) > 500_000_000:
        raise ValueError('Archive exceeds limits')
    archive.extractall('payload', members=members, filter='data')
PY
app=payload/Ruffle.app
[[ -d $app && ! -L $app ]]
plist="$app/Contents/Info.plist"
for key in CFBundleIdentifier CFBundleShortVersionString CFBundleVersion CFBundleExecutable LSMinimumSystemVersion; do
  value=$(/usr/bin/plutil -extract "$key" raw "$plist" 2>/dev/null || true)
  printf '%s=%s\n' "$key" "$value"
done
bin=$(/usr/bin/plutil -extract CFBundleExecutable raw "$plist")
[[ "$bin" =~ ^[[:alnum:]._-]+$ ]]
/usr/bin/file "$app/Contents/MacOS/$bin"
/usr/bin/lipo "$app/Contents/MacOS/$bin" -verify_arch arm64
/usr/bin/vtool -arch arm64 -show-build "$app/Contents/MacOS/$bin"
/usr/bin/codesign --display --verbose=4 "$app" 2>&1 | /usr/bin/grep -E '^(Authority|TeamIdentifier|Identifier|Format|Timestamp|Signature)='
/usr/bin/codesign --verify --deep --strict --all-architectures "$app"
/usr/bin/codesign --verify --check-notarization -R=notarized "$app"
/usr/sbin/spctl --assess --ignore-cache --no-cache --type execute --verbose=4 "$app"
printf 'PASS RUFFLE %s\n' "$tag"
