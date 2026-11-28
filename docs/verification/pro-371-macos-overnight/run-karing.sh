#!/bin/bash
set -euo pipefail
umask 077
root=${1:?Owned Mac verification directory}
[[ $(uname -m) == arm64 && "$root" == /private/tmp/shift-air-overnight.* && -O "$root" && -f "$root/.shift-owned" ]]
profile=$(mktemp -d "$root/profile-karing.XXXXXXXX")
trap 'if [[ "$profile" == "$root/profile-karing."* && -f "$profile/.shift-overnight-cycle" ]]; then /usr/bin/find "$profile" -xdev -depth -delete; fi' EXIT
touch "$profile/.shift-overnight-cycle"
printf 'synthetic user data\n' > "$profile/synthetic-user-data.txt"
mkdir -p "$profile/"{tmp,Applications,.Trash,Library/Caches/ShiftAnything}
cd "$root"
python3 - "$profile/Library/Caches/ShiftAnything/releases.json" <<'PY'
from pathlib import Path
import json, sys, time

repo = 'KaringX/karing'
rows = [
    ('2932', 116879271, '3631628da559b802aebb8435431f2f0aa01da132cb2a87985abc636fcb4369fc'),
    ('2934', 116908954, 'eade8421f83695ce440709fb07a477995fa64ff0608e6180d6e4cca1bf7bc79e'),
]
releases, packages = [], []
for build, size, digest in rows:
    tag = 'v1.2.26.' + build
    name = 'karing_1.2.26.' + build + '_macos_universal.dmg'
    url = f'https://github.com/{repo}/releases/download/{tag}/{name}'
    packages.append(f'{tag}\t{name}\t{size}\t{digest}\t{url}\n')
    releases.append({'tag': tag, 'version': '1.2.26+karing-revision.' + build, 'name': tag, 'published_at': None,
                     'html_url': f'https://github.com/{repo}/releases/tag/{tag}', 'notes': '',
                     'prerelease': False, 'assets': [{'name': name, 'size': size, 'sha256': digest, 'url': url}]})
Path('karing.tsv').write_text(''.join(packages))
Path(sys.argv[1]).write_text(json.dumps({'entries': {}, 'versions': {repo: [int(time.time()), releases[::-1]]}}) + '\n')
PY
sw_vers -productVersion
spctl --status
run_phase() {
    env -i HOME="$profile" CFFIXED_USER_HOME="$profile" TMPDIR="$profile/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$root/qa-karing-cycle" karing "$1" Karing
}
bundle="$profile/Applications/Karing.app"
for phase in install update; do
    run_phase "$phase"
    /usr/bin/lipo "$bundle/Contents/MacOS/Karing" -verify_arch arm64
    /usr/bin/codesign --verify --all-architectures --deep --strict "$bundle"
    /usr/bin/codesign --verify --check-notarization -R='notarized and identifier "com.nebula.karing" and certificate leaf[subject.OU] = "TNPM9PFX3W"' "$bundle"
    /usr/sbin/spctl -a -vv -t execute "$bundle"
    /usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$bundle/Contents/Info.plist"
    /usr/libexec/PlistBuddy -c 'Print :CFBundleVersion' "$bundle/Contents/Info.plist"
    printf 'PASS ARCHITECTURE SIGNATURE GATEKEEPER %s\n' "$phase"
done
run_phase remove
test -d "$profile/.Trash/Karing.app"
test "$(cat "$profile/synthetic-user-data.txt")" = 'synthetic user data'
printf 'PASS NATIVE CYCLE karing (1.2.26.2932 → 1.2.26.2934), private Trash and synthetic data preserved\n'
