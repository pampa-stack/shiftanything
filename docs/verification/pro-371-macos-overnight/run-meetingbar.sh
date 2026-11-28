#!/bin/bash
set -euo pipefail
umask 077
root=${1:?Owned Mac verification directory}
[[ $(uname -m) == arm64 && "$root" == /private/tmp/shift-air-overnight.* && -O "$root" && -f "$root/.shift-owned" ]]
profile=$(mktemp -d "$root/profile-meetingbar.XXXXXXXX")
trap 'if [[ "$profile" == "$root/profile-meetingbar."* && -f "$profile/.shift-overnight-cycle" ]]; then /usr/bin/find "$profile" -xdev -depth -delete; fi' EXIT
touch "$profile/.shift-overnight-cycle"
printf 'synthetic user data\n' > "$profile/synthetic-user-data.txt"
mkdir -p "$profile/"{tmp,Applications,.Trash,Library/Caches/ShiftAnything}
cd "$root"
python3 - "$profile/Library/Caches/ShiftAnything/releases.json" <<'PY'
from pathlib import Path
import json, sys, time

repo = 'leits/MeetingBar'
rows = [
    ('v4.11.6', '4.11.6', 11294412, '4f19af496d4ff44b9b0ee02be02bc76207f3e365a7f9e77c7219a08c607838d7'),
    ('v5.0.0', '5.0.1', 8210845, '26d4759f73f429c2aa93d225704bf85cdde8b1c035c12aadeb63d4dddbdae1e4'),
]
name = 'MeetingBar.dmg'
releases, packages = [], []
for tag, version, size, digest in rows:
    url = f'https://github.com/{repo}/releases/download/{tag}/{name}'
    packages.append(f'{tag}\t{name}\t{size}\t{digest}\t{url}\n')
    releases.append({'tag': tag, 'version': version, 'name': tag, 'published_at': None,
                     'html_url': f'https://github.com/{repo}/releases/tag/{tag}', 'notes': '',
                     'prerelease': False, 'assets': [{'name': name, 'size': size, 'sha256': digest, 'url': url}]})
Path('meetingbar.tsv').write_text(''.join(packages))
Path(sys.argv[1]).write_text(json.dumps({'entries': {}, 'versions': {repo: [int(time.time()), releases[::-1]]}}) + '\n')
PY
sw_vers -productVersion
spctl --status
run_phase() {
    env -i HOME="$profile" CFFIXED_USER_HOME="$profile" TMPDIR="$profile/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$root/native-cycle-meetingbar" meetingbar "$1" MeetingBar
}
bundle="$profile/Applications/MeetingBar.app"
for phase in install update; do
    run_phase "$phase"
    /usr/bin/lipo "$bundle/Contents/MacOS/MeetingBar" -verify_arch arm64
    /usr/bin/codesign --verify --all-architectures --deep --strict "$bundle"
    /usr/bin/codesign --verify --check-notarization -R='notarized and identifier "leits.MeetingBar" and certificate leaf[subject.OU] = "KGH289N6T8"' "$bundle"
    /usr/sbin/spctl -a -vv -t execute "$bundle"
    /usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$bundle/Contents/Info.plist"
    printf 'PASS ARCHITECTURE SIGNATURE GATEKEEPER %s\n' "$phase"
done
run_phase remove
test -d "$profile/.Trash/MeetingBar.app"
test "$(cat "$profile/synthetic-user-data.txt")" = 'synthetic user data'
printf 'PASS NATIVE CYCLE meetingbar (4.11.6 → 5.0.1), private Trash and synthetic data preserved\n'
