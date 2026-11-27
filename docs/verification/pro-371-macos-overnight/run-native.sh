#!/bin/bash
set -euo pipefail
umask 077
cd "$(dirname "$0")"
root="$(pwd -P)"
case "$root" in /private/tmp/shift-air-overnight.*) ;; *) exit 2 ;; esac
test -f .shift-owned
test "$(uname -m)" = arm64
id="$1"
repo="$2"
app="$3"
old="$4"
new="$5"
case "$id" in moonlight|openpencil|airi|sunshine|openttd|rectangle|codeedit|maccy|blender|gimp) ;; *) exit 2 ;; esac
for tag in "$old" "$new"; do
    log_id="$id"
    [[ "$id" != openpencil ]] || log_id=open-pencil
    grep -x -F "PASS PREFLIGHT $log_id $tag" "$log_id-$tag.log"
done
df -k "$root" | awk 'NR==2{print; exit !($4>=8388608)}'
profile="$(mktemp -d "$root/profile-$id.XXXXXXXX")"
mkdir -p "$profile/Applications" "$profile/.Trash" "$profile/tmp" "$profile/Library/Caches/ShiftAnything"
touch "$profile/.shift-overnight-cycle"
printf 'synthetic user data\n' > "$profile/synthetic-user-data.txt"
trap 'if [[ "$profile" == "$root/profile-$id."* ]] && test -f "$profile/.shift-overnight-cycle"; then /bin/rm -rf -- "$profile"; fi' EXIT
python3 - "$id" "$repo" "$old" "$new" "$profile/Library/Caches/ShiftAnything/releases.json" <<'PY'
from pathlib import Path
import json, re, sys, time

app_id, repo, old, new, cache = sys.argv[1:]
log_id = 'open-pencil' if app_id == 'openpencil' else app_id
tsv = []
releases = []
for tag in (old, new):
    [digest, size, url] = re.search(r'^ARTIFACT ([a-f0-9]{64}) (\d+) (https://\S+)$', Path(f'{log_id}-{tag}.log').read_text(), re.M).groups()
    name = url.rsplit('/', 1)[-1]
    if app_id == 'gimp':
        assert re.fullmatch(r'\d+\.\d+\.\d+', tag)
        series = tag.rsplit('.', 1)[0]
        assert name == f'gimp-{tag}-arm64.dmg'
        assert url == f'https://download.gimp.org/gimp/v{series}/macos/{name}'
        [release] = [item for item in json.loads(Path('gimp-versions.json').read_text())['STABLE'] if item['version'] == tag]
        [asset] = [item for item in release['macos'] if item['filename'] == name]
        published_digests = {asset['sha256']}
        assert published_digests == {digest}, published_digests
        version = tag
    elif app_id == 'blender':
        assert re.fullmatch(r'\d+\.\d+\.\d+', tag)
        series = tag.rsplit('.', 1)[0]
        assert name == f'blender-{tag}-macos-arm64.dmg'
        assert url == f'https://download.blender.org/release/Blender{series}/{name}'
        checksum = Path(f'blender-{tag}.sha256').read_text()
        published_digests = set(re.findall(rf'^([a-f0-9]{{64}})\s+\*?{re.escape(name)}\s*$', checksum, re.M))
        assert published_digests == {digest}, published_digests
        version = tag
    elif app_id == 'openttd':
        assert re.fullmatch(r'\d+\.\d+', tag)
        assert url == f'https://cdn.openttd.org/openttd-releases/{tag}/openttd-{tag}-macos-universal.dmg'
        published_digests = set()
        if tag == new:
            official = Path('openttd-stable.html').read_text()
            link = official.index(url)
            assert digest in official[link:link+3000]
            published_digests = {digest}
        version = f'{tag}.0'
    else:
        assert re.fullmatch(r'\d+\.\d+\.\d+' if app_id == 'maccy' else r'v\d+\.\d+\.\d+', tag)
        assert url == f'https://github.com/{repo}/releases/download/{tag}/{name}'
        html = Path(f'{app_id}-{tag}-assets.html').read_text()
        [block] = [item for item in html.split('</li>') if f'href="/{repo}/releases/download/{tag}/{name}"' in item]
        published_digests = set(re.findall(r'sha256:([a-f0-9]{64})', block))
        if (app_id, tag) == ("moonlight", "v6.1.0"):
            assert not published_digests
        else:
            assert published_digests == {digest}, published_digests
        version = tag.removeprefix('v')
    releases.append({'tag': tag, 'version': version, 'name': tag, 'published_at': None, 'html_url': f'https://github.com/{repo}/releases/tag/{tag}', 'notes': '', 'prerelease': False, 'assets': [{'name': name, 'size': int(size), 'sha256': digest, 'url': url}]})
    tsv.append(f'{tag}\t{name}\t{size}\t{digest}\t{url}\n')
    print(f'OFFICIAL INDEX {app_id} {tag} digest={"published" if published_digests else "computed, signer/notarization corroborated"}')
Path(f'{app_id}.tsv').write_text("".join(tsv))
if app_id not in ('blender', 'gimp'):
    Path(cache).write_text(json.dumps({'entries': {}, 'versions': {repo: [int(time.time()), releases[::-1]]}}) + '\n')
PY
sw_vers -productVersion
launchctl managername
spctl --status
for phase in install update remove; do
    env -i HOME="$profile" CFFIXED_USER_HOME="$profile" TMPDIR="$profile/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$root/native-cycle" "$id" "$phase" "$app"
    [[ "$(cat "$profile/synthetic-user-data.txt")" == "synthetic user data" ]]
    if [[ ( "$id" == openttd || "$id" == blender || "$id" == gimp ) && "$phase" != remove ]]; then
        python3 - "$id" "$profile" "$phase" "$old" "$new" <<'PYLAUNCH'
from pathlib import Path
import re, subprocess, sys
app_id, profile, phase, old, new = sys.argv[1:]
version = old if phase == 'install' else new
bundle, binary, flag = {'openttd': ('OpenTTD', 'openttd', '-h'), 'blender': ('Blender', 'Blender', '--version'), 'gimp': ('GIMP', 'gimp-3.2', '--version')}[app_id]
executable = Path(profile) / f'Applications/{bundle}.app/Contents/MacOS/{binary}'
command = ['/usr/bin/sandbox-exec', '-p', '(version 1) (allow default) (deny network*) (deny file-write*)', str(executable), flag]
env = {'HOME': profile, 'CFFIXED_USER_HOME': profile, 'TMPDIR': f'{profile}/tmp', 'PATH': '/usr/bin:/bin'}
output = subprocess.run(command, env=env, check=True, capture_output=True, text=True, timeout=15).stdout
assert re.search(rf'\b{re.escape(version)}\b', output.splitlines()[0]), output[:160]
print(f'PASS CLI {binary} {flag}: {output.splitlines()[0]}')
PYLAUNCH
    fi
done
test -d "$profile/.Trash/$app.app"
printf 'PASS NATIVE CYCLE %s (%s → %s), private Trash and synthetic data preserved\n' "$id" "$old" "$new"
