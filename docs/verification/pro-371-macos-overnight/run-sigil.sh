#!/bin/bash
set -euo pipefail
umask 077
root="$1"
[[ "$(uname -m)" == arm64 && "$root" == /private/tmp/shift-air-overnight.* && -O "$root" && -f "$root/.shift-owned" ]]
if /usr/bin/pgrep -x Sigil >/dev/null; then printf 'Sigil already running; leaving it alone\n' >&2; exit 2; fi
profile="$(mktemp -d "$root/profile-sigil.XXXXXXXX")"
trap 'if [[ "$profile" == "$root/profile-sigil."* && -f "$profile/.shift-overnight-cycle" ]] && ! /bin/ps -axww -o comm= | /usr/bin/grep -Fxq "$profile/Applications/Sigil.app/Contents/MacOS/Sigil"; then /usr/bin/find "$profile" -xdev -depth -delete; fi' EXIT
touch "$profile/.shift-overnight-cycle"
printf 'synthetic user data\n' > "$profile/synthetic-user-data.txt"
mkdir -p "$profile/"{tmp,Applications,.Trash,Library/Caches/ShiftAnything}
cd "$root"
python3 - "$profile/Library/Caches/ShiftAnything/releases.json" <<'PY'
from pathlib import Path
import json, sys, time

repo = 'Sigil-Ebook/Sigil'
rows = [
    ('2.8.1', 129467312, '285addb9abee1d69c913d6a6b504240946b5274fd2193942ed59697536279afe'),
    ('2.8.6', 132137600, '3be3be094745dfec892a02ac2f8d90934d4da2dc595ec1a95a8525f9ad27efd6'),
]
releases = []
tsv = []
for tag, size, digest in rows:
    name = f'Sigil.app-{tag}-Mac-arm64.txz'
    url = f'https://github.com/{repo}/releases/download/{tag}/{name}'
    tsv.append(f'{tag}\t{name}\t{size}\t{digest}\t{url}\n')
    releases.append({'tag': tag, 'version': tag, 'name': tag, 'published_at': None,
                     'html_url': f'https://github.com/{repo}/releases/tag/{tag}', 'notes': '',
                     'prerelease': False, 'assets': [{'name': name, 'size': size, 'sha256': digest, 'url': url}]})
Path('sigil.tsv').write_text(''.join(tsv))
Path(sys.argv[1]).write_text(json.dumps({'entries': {}, 'versions': {repo: [int(time.time()), releases[::-1]]}}) + '\n')
PY
sw_vers -productVersion
spctl --status
run_phase() {
    env -i HOME="$profile" CFFIXED_USER_HOME="$profile" TMPDIR="$profile/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$root/native-cycle" sigil "$1" Sigil
}
for phase in install update; do
    run_phase "$phase"
    bundle="$profile/Applications/Sigil.app"
    /usr/bin/lipo "$bundle/Contents/MacOS/Sigil" -verify_arch arm64
    /usr/bin/codesign --verify --all-architectures --deep --strict "$bundle"
    /usr/bin/codesign --verify --check-notarization -R='notarized and identifier "com.sigil-ebook.Sigil.app" and certificate leaf[subject.OU] = "2SMCVQU3CJ"' "$bundle"
    /usr/sbin/spctl -a -vv -t execute "$bundle"
    if /usr/bin/pgrep -x Sigil >/dev/null; then printf 'Another Sigil instance started; leaving it alone\n' >&2; exit 2; fi
    run_phase open
    python3 - "$bundle/Contents/MacOS/Sigil" <<'PY'
import os, signal, subprocess, sys, time
executable = sys.argv[1]
def exact_processes():
    output = subprocess.check_output(['/bin/ps', '-axww', '-o', 'pid=,comm='], text=True)
    return {int(pid) for line in output.splitlines() if line.strip()
            for pid, command in [line.strip().split(maxsplit=1)] if command == executable}
for _ in range(75):
    matches = exact_processes()
    if len(matches) == 1:
        break
    time.sleep(.2)
else:
    raise SystemExit('Expected one process from the isolated Sigil bundle')
[pid] = matches
time.sleep(2)
assert pid in exact_processes(), 'The isolated app exited immediately'
os.kill(pid, signal.SIGTERM)
for _ in range(50):
    if pid not in exact_processes():
        break
    time.sleep(.2)
else:
    raise SystemExit('The isolated app did not exit')
print('PASS OPEN AND CLOSE Sigil executable in the isolated profile')
PY
done
run_phase remove
test -d "$profile/.Trash/Sigil.app"
test "$(cat "$profile/synthetic-user-data.txt")" = 'synthetic user data'
printf 'PASS NATIVE CYCLE sigil (2.8.1 → 2.8.6), private Trash and synthetic data preserved\n'
