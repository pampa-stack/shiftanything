#!/bin/bash
set -euo pipefail
umask 077
root="$1"
[[ "$(uname -m)" == arm64 && "$root" == /private/tmp/shift-air-overnight.* && -O "$root" && -f "$root/.shift-owned" ]]
profile="$(mktemp -d "$root/profile-syncthing.XXXXXXXX")"
trap 'if [[ "$profile" == "$root/profile-syncthing."* && -f "$profile/.shift-overnight-cycle" ]]; then /usr/bin/find "$profile" -xdev -depth -delete; fi' EXIT
touch "$profile/.shift-overnight-cycle"
printf 'synthetic user data\n' > "$profile/synthetic-user-data.txt"
mkdir -p "$profile/"{tmp,Applications,.Trash,Library/Caches/ShiftAnything}
cd "$root"
python3 - "$profile/Library/Caches/ShiftAnything/releases.json" <<'PY'
from pathlib import Path
import json, sys, time

repo = 'syncthing/syncthing-macos'
rows = [
    ('2.1.5', 28061360, '50903a2a6221856e1c36e3c5c5ff39e897053d29aa1f685ad576622c93069e88'),
    ('2.1.6', 28069114, 'f81d091bb19c42b3bb7a2fda9e80f1794cdb27fadb18b094b0c99b53ef571e2d'),
]
releases = []
tsv = []
for version, size, digest in rows:
    tag = f'v{version}-1'
    name = f'Syncthing-{version}-1.dmg'
    url = f'https://github.com/{repo}/releases/download/{tag}/{name}'
    tsv.append(f'{tag}\t{name}\t{size}\t{digest}\t{url}\n')
    releases.append({'tag': tag, 'version': version, 'name': tag, 'published_at': None,
                     'html_url': f'https://github.com/{repo}/releases/tag/{tag}', 'notes': '',
                     'prerelease': False, 'assets': [{'name': name, 'size': size, 'sha256': digest, 'url': url}]})
Path('syncthing-macos.tsv').write_text(''.join(tsv))
Path(sys.argv[1]).write_text(json.dumps({'entries': {}, 'versions': {repo: [int(time.time()), releases[::-1]]}}) + '\n')
PY
sw_vers -productVersion
spctl --status
run_phase() {
    env -i HOME="$profile" CFFIXED_USER_HOME="$profile" TMPDIR="$profile/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$root/native-cycle-syncthing" syncthing-macos "$1" Syncthing
}
for phase in install update; do
    run_phase "$phase"
    bundle="$profile/Applications/Syncthing.app"
    /usr/bin/lipo "$bundle/Contents/MacOS/Syncthing" -verify_arch arm64
    /usr/bin/codesign --verify --all-architectures --deep --strict "$bundle"
    /usr/bin/codesign --verify --check-notarization -R='notarized and identifier "com.github.xor-gate.syncthing-macosx" and certificate leaf[subject.OU] = "LQE5SYM783"' "$bundle"
    /usr/sbin/spctl -a -vv -t execute "$bundle"
    /usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$bundle/Contents/Info.plist"
    printf 'PASS ARCHITECTURE SIGNATURE GATEKEEPER %s\n' "$phase"
done
run_phase remove
test -d "$profile/.Trash/Syncthing.app"
test "$(cat "$profile/synthetic-user-data.txt")" = 'synthetic user data'
printf 'PASS NATIVE CYCLE syncthing-macos (2.1.5-1 → 2.1.6-1), private Trash and synthetic data preserved\n'
