#!/bin/bash
set -euo pipefail
umask 077
cd "$(dirname "$0")"
root="$(pwd -P)"
case "${root%/*}:${root##*/}" in /private/tmp:shift-air-overnight.*|/Volumes/T7-Backup:shift-air-overnight.*) ;; *) exit 2 ;; esac
test -f .shift-owned
test "$(uname -m)" = arm64
id="$1"
repo="$2"
app="$3"
old="$4"
new="$5"
case "$id" in craft-anything|moonlight|openpencil|airi|sunshine|openttd|rectangle|codeedit|maccy|blender|gimp|netnewswire|krita|vlc|inks|kdenlive|subtitle-edit|openwhispr|deepchat|alttab|linearmouse|loop-macos|dyad|finicky|hammerspoon|ice|virtualbuddy|easydict|utm|openmtp|invoke-community|mist|coteditor|monitorcontrol|gnucash|floorp|stellarium|macshot|signal-desktop|firefox|element|freeplane|amethyst|dockdoor|macvim|hidden-bar|swiftbar|sloth|jellyfin-desktop|iterm2|itsycal|github-desktop|aionui|ruffle|scummvm|raspberry-pi-imager|wwdc) ;; *) exit 2 ;; esac
for tag in "$old" "$new"; do
    log_id="$id"
    [[ "$id" != openpencil ]] || log_id=open-pencil
    [[ "$id" != inks ]] || log_id=inkscape
    if [[ "$id" == scummvm ]]; then
        grep -x -F "PASS PREFLIGHT $log_id ${tag#v}" "$log_id-${tag#v}.log"
    elif [[ "$id" == ruffle ]]; then
        grep -x -F "PASS RUFFLE $tag" "$log_id-$tag.log"
    else
        grep -x -F "PASS PREFLIGHT $log_id $tag" "$log_id-$tag.log"
    fi
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
log_id = {'openpencil': 'open-pencil', 'inks': 'inkscape'}.get(app_id, app_id)
tsv = []
releases = []
for tag in (old, new):
    proof_tag = tag.removeprefix('v') if app_id == 'scummvm' else tag
    [digest, size, url] = re.search(r'^ARTIFACT ([a-f0-9]{64}) (\d+) (https://\S+)$', Path(f'{log_id}-{proof_tag}.log').read_text(), re.M).groups()
    name = url.rsplit('/', 1)[-1]
    if app_id == 'inks':
        assert re.fullmatch(r'\d+\.\d+\.\d+', tag)
        assert name == f'Inkscape-{tag}_arm64.dmg'
        assert url == f'https://media.inkscape.org/dl/resources/file/{name}'
        checksum, published_name = Path(f'inkscape-{tag}.sha256').read_text().strip().split(maxsplit=1)
        expected_name = 'Inkscape-1.4.3+33103_arm64.dmg' if tag == '1.4.3' else name
        assert checksum == digest and published_name == expected_name
        if tag == new:
            assert Path('inkscape-stable-location.txt').read_text().strip() == f'/release/inkscape-{tag}/'
        published_digests = {digest}
        version = tag
    elif app_id == 'itsycal':
        assert tag in ('0.15.14', '0.15.15')
        assert name == f'Itsycal-{tag}.zip'
        assert url == f'https://itsycal.s3.amazonaws.com/{name}'
        signature = Path(f'itsycal-{tag}.log').read_text().splitlines()
        assert 'CFBundleIdentifier=com.mowglii.ItsycalApp' in signature
        assert f'CFBundleShortVersionString={tag}' in signature
        assert 'TeamIdentifier=HFT3T55WND' in signature
        assert 'source=Notarized Developer ID' in signature
        if tag == new:
            import xml.etree.ElementTree as ET
            ns = {'sparkle': 'http://www.andymatuschak.org/xml-namespaces/sparkle'}
            [item] = [item for item in ET.parse('itsycal.xml').iter('item')
                      if item.find('enclosure').get('url') == f'https://s3.amazonaws.com/itsycal/{name}']
            assert item.find('enclosure').get('length') == size
            assert item.findtext('sparkle:minimumSystemVersion', namespaces=ns) == '11.0'
        published_digests = set()
        version = tag
    elif app_id == 'iterm2':
        assert re.fullmatch(r'\d+\.\d+\.\d+', tag)
        assert name == f"iTerm2-{tag.replace('.', '_')}.zip"
        assert url == f'https://iterm2.com/downloads/stable/{name}'
        release_section = Path('iterm2-downloads.html').read_text().split(url + '"', 1)[1].split('<h4', 1)[0]
        assert re.search(r'Hash: SHA256\s+([a-f0-9]{64})\b', release_section)[1] == digest
        if tag == new:
            import xml.etree.ElementTree as ET
            [asset] = [item for item in ET.parse('final_modern.xml').iter('enclosure') if item.get('url') == url]
            assert asset.get('length') == size
        published_digests = {digest}
        version = tag
    elif app_id == 'aionui':
        assert repo == 'iOfficeAI/AionUi'
        assert re.fullmatch(r'v\d+\.\d+\.\d+', tag)
        version = tag.removeprefix('v')
        assert name == f'AionUi-{version}-mac-arm64.dmg'
        assert url == f'https://static.aionui.com/releases/{version}/{name}'
        signature = Path(f'{app_id}-{tag}.log').read_text().splitlines()
        assert 'CFBundleIdentifier=com.aionui.app' in signature
        assert f'CFBundleShortVersionString={version}' in signature
        assert 'TeamIdentifier=52JQX2HUSC' in signature
        assert 'source=Notarized Developer ID' in signature
        if tag == new:
            feed = Path('aionui-macos-feed.yml').read_text()
            assert re.search(r'^version: (.+)$', feed, re.M)[1] == version
            [asset] = [block for block in feed.split('  - url: ') if block.startswith(name + '\n')]
            assert re.search(r'^    size: (\d+)$', asset, re.M)[1] == size
        published_digests = set()
    elif app_id == 'scummvm':
        assert repo == 'scummvm/scummvm'
        assert re.fullmatch(r'v\d{4}\.\d+\.\d+', tag)
        version = tag.removeprefix('v')
        assert name == f'scummvm-{version}-macosx.dmg'
        assert url == f'https://downloads.scummvm.org/frs/scummvm/{version}/{name}'
        assert Path(f'{name}.sha256').read_text().strip() == digest
        if tag == new:
            assert url in Path('scummvm-downloads.html').read_text()
        published_digests = {digest}
    elif app_id == 'element':
        assert re.fullmatch(r'\d+\.\d+\.\d+', tag)
        name = f'Element-{tag}-universal.dmg'
        assert url == f'https://packages.element.io/desktop/install/macos/{name}'
        official = Path('element-index.html').read_text()
        assert f'href="{name}"' in official
        if tag == new:
            versions = re.findall(r'href="Element-(\d+\.\d+\.\d+)-universal\.dmg"', official)
            assert max(versions, key=lambda value: tuple(map(int, value.split('.')))) == tag
        published_digests = set()
        version = tag
    elif app_id == 'firefox':
        assert re.fullmatch(r'\d+\.\d+(?:\.\d+)?', tag)
        assert url == f'https://archive.mozilla.org/pub/firefox/releases/{tag}/mac/en-US/Firefox%20{tag}.dmg'
        name = f'Firefox-{tag}.dmg'
        [published_digest] = [
            line[:64] for line in Path(f'firefox-{tag}.sha256').read_text().splitlines()
            if line[64:] == f'  mac/en-US/Firefox {tag}.dmg'
        ]
        assert published_digest == digest
        if tag == new:
            assert json.loads(Path('firefox-versions.json').read_text())['LATEST_FIREFOX_VERSION'] == tag
        published_digests = {digest}
        version = tag if tag.count('.') == 2 else f'{tag}.0'
    elif app_id == 'signal-desktop':
        assert re.fullmatch(r'\d+\.\d+\.\d+', tag)
        assert name == f'signal-desktop-mac-universal-{tag}.dmg'
        assert url == f'https://updates.signal.org/desktop/{name}'
        if tag == new:
            manifest = Path('signal-mac-stable.yml').read_text().splitlines()
            assert [line for line in manifest if line.startswith('version: ')] == [f'version: {tag}']
            assert f'  - url: {name}' in manifest
        published_digests = set()
        version = tag
    elif app_id == 'kdenlive':
        assert re.fullmatch(r'\d+\.\d{2}\.\d+', tag)
        series = tag.rsplit('.', 1)[0]
        assert name == f'kdenlive-{tag}-arm64.dmg'
        assert url == f'https://cdn.download.kde.org/stable/kdenlive/{series}/macOS/{name}'
        assert Path(f'kdenlive-{tag}.sha256').read_text().split()[0] == digest
        if tag == new:
            assert f'<item><title>Release {tag} of Kdenlive</title><link>https://apps.kde.org/kdenlive/#{tag}</link>' in Path('kdenlive-releases.xml').read_text()
        published_digests = {digest}
        version = '.'.join(str(int(part)) for part in tag.split('.'))
    elif app_id == 'vlc':
        assert re.fullmatch(r'\d+\.\d+\.\d+', tag)
        assert name == f'vlc-{tag}-arm64.dmg'
        assert url == f'https://download.videolan.org/pub/videolan/vlc/{tag}/macosx/{name}'
        checksum = Path(f'vlc-{tag}.sha256').read_text()
        published_digests = set(re.findall(rf'^([a-f0-9]{{64}})\s+\*?{re.escape(name)}\s*$', checksum, re.M))
        assert published_digests == {digest}, published_digests
        if tag == new:
            assert f'<a href="{name}">' in Path('vlc-mac-stable.html').read_text()
        version = tag
    elif app_id == 'krita':
        assert re.fullmatch(r'\d+\.\d+\.\d+(?:\.1)?', tag)
        assert url == f'https://download.kde.org/stable/krita/{tag}/krita-{tag}-signed.dmg'
        mirrorlist = Path(f'krita-{tag}.mirrorlist').read_text()
        checksum = re.search(r'>SHA256</a></td><td[^>]*>([a-f0-9]{64})</td>', mirrorlist)
        assert checksum is not None and checksum.group(1) == digest
        assert f'({int(size):,} bytes)' in mirrorlist or f'({size} bytes)' in mirrorlist
        if tag == new:
            html = Path('krita-download.html').read_text()
            assert re.search(r' href=(?:"' + re.escape(url) + r'"|' + re.escape(url) + r'(?=\s|>))', html)
        published_digests = {digest}
        version = tag if tag != '5.3.4.1' else '5.3.4'
    elif app_id == 'gimp':
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
    elif app_id == 'wwdc':
        assert repo == 'insidegui/WWDC'
        assert tag in ('7.4.2', '7.5')
        build = {'7.4.2': '1040', '7.5': '1044'}[tag]
        assert name == f'WWDC_v{tag}-{build}.dmg'
        assert url == f'https://github.com/{repo}/releases/download/{tag}/{name}'
        html = Path(f'wwdc-{tag}-assets.html').read_text()
        assert f'href="/{repo}/releases/download/{tag}/{name}"' in html
        signature = Path(f'wwdc-{tag}.log').read_text()
        for fact in (f'CFBundleShortVersionString {tag}', 'TeamIdentifier=8C7439RJLG', 'source=Notarized Developer ID'):
            assert fact in signature.splitlines(), fact
        published_digests = set()
        if tag == '7.5':
            cask = json.loads(Path('wwdc-cask.json').read_text())
            assert cask['url'] == url and cask['sha256'] == digest
        version = '7.5.0' if tag == '7.5' else tag
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
        tag_pattern = {
            'maccy': r'\d+\.\d+\.\d+',
            'sloth': r'\d+\.\d+',
            'macvim': r'release-\d+',
            'github-desktop': r'release-\d+\.\d+\.\d+',
            'dockdoor': r'\d+\.\d+\.\d+',
            'easydict': r'\d+\.\d+\.\d+',
            'utm': r'v(?:5\.0\.8|5\.0\.9-2)',
            'mist': r'v\d+\.\d+',
            'gnucash': r'\d+\.\d+',
            'stellarium': r'v\d+\.\d+',
            'coteditor': r'\d+\.\d+\.\d+',
            'freeplane': r'release-\d+\.\d+\.\d+',
            'hammerspoon': r'\d+\.\d+\.\d+',
            'ice': r'\d+\.\d+\.\d+',
            'virtualbuddy': r'\d+\.\d+(?:\.\d+)?',
            'loop-macos': r'\d+\.\d+\.\d+',
            'netnewswire': r'mac-\d+\.\d+(?:\.\d+)?',
        }.get(app_id, r'v\d+\.\d+\.\d+')
        assert re.fullmatch(tag_pattern, tag)
        assert url == f'https://github.com/{repo}/releases/download/{tag}/{name}'
        html = Path(f'{app_id}-{tag}-assets.html').read_text()
        [block] = [item for item in html.split('</li>') if f'href="/{repo}/releases/download/{tag}/{name}"' in item]
        published_digests = set(re.findall(r'sha256:([a-f0-9]{64})', block))
        if (app_id, tag) in {("moonlight", "v6.1.0"), ("ice", "0.11.11"), ("ice", "0.11.12"), ("openmtp", "v3.2.25"), ("monitorcontrol", "v4.2.0"), ("jellyfin-desktop", "v1.11.1"), ("jellyfin-desktop", "v1.12.0")}:
            assert not published_digests
        else:
            assert published_digests == {digest}, published_digests
        version = tag.removeprefix('v').removeprefix('mac-').removeprefix('release-')
        if app_id == 'macvim':
            version = re.search(r'^CFBundleShortVersionString=(\d+\.\d+\.\d+)$', Path(f'{app_id}-{tag}.log').read_text(), re.M)[1]
        if app_id == 'utm' and tag == 'v5.0.9-2':
            version = '5.0.9'
        if app_id in ('netnewswire', 'virtualbuddy', 'mist', 'gnucash', 'stellarium', 'sloth') and version.count('.') == 1:
            version += '.0'
    releases.append({'tag': tag, 'version': version, 'name': tag, 'published_at': None, 'html_url': f'https://github.com/{repo}/releases/tag/{tag}', 'notes': '', 'prerelease': False, 'assets': [{'name': name, 'size': int(size), 'sha256': digest, 'url': url}]})
    tsv.append(f'{tag}\t{name}\t{size}\t{digest}\t{url}\n')
    print(f'OFFICIAL INDEX {app_id} {tag} digest={"published" if published_digests else "computed, signer/notarization corroborated"}')
Path(f'{app_id}.tsv').write_text("".join(tsv))
if app_id not in ('blender', 'gimp', 'netnewswire', 'krita', 'vlc', 'inks', 'kdenlive', 'iterm2', 'itsycal', 'github-desktop', 'signal-desktop', 'firefox', 'element'):
    Path(cache).write_text(json.dumps({'entries': {}, 'versions': {repo: [int(time.time()), releases[::-1]]}}) + '\n')
PY
sw_vers -productVersion
launchctl managername
spctl --status
run_phase() {
    if [[ "$id" == github-desktop ]]; then
        df -k "$root" | awk 'NR==2{print; exit !($4>=12582912)}'
    fi
    env -i HOME="$profile" CFFIXED_USER_HOME="$profile" TMPDIR="$profile/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$root/native-cycle" "$id" "$1" "$app"
}
phases=(install update remove)
if [[ "$id" == iterm2 || "$id" == github-desktop || "$id" == monitorcontrol ]]; then
    run_phase install-latest
    run_phase remove
    test -d "$profile/.Trash/$app.app"
    test -f "$profile/.shift-overnight-cycle"
    /bin/rm -rf -- "$profile/.Trash/$app.app"
    # The previous release is a validated migration fixture, never a launcher download choice.
    IFS=$'	' read -r old_tag filename size digest url < "$id.tsv"
    case "$id" in
        iterm2)
            [[ "$old_tag" == 3.7.8 && "$app" == iTerm && "$url" == https://iterm2.com/downloads/stable/iTerm2-3_7_8.zip ]]
            version=3.7.8
            bin=iTerm2
            requirement='notarized and identifier "com.googlecode.iterm2" and certificate leaf[subject.OU] = "H7V7XYVQ7D"'
            ;;
        github-desktop)
            [[ "$old_tag" == release-3.7.0 && "$app" == 'GitHub Desktop' && "$url" == https://github.com/desktop/desktop/releases/download/release-3.7.0/GitHub.Desktop-arm64.zip ]]
            version=3.7.0
            bin='GitHub Desktop'
            requirement='notarized and identifier "com.github.GitHubClient" and certificate leaf[subject.OU] = "VEKTX9H2N7"'
            ;;
        monitorcontrol)
            [[ "$old_tag" == v4.2.0 && "$app" == MonitorControl && "$url" == https://github.com/MonitorControl/MonitorControl/releases/download/v4.2.0/MonitorControl.4.2.0.dmg ]]
            [[ "$digest" == 43deae2bac157b50f6b8bbd62fea4547a2dc9ab53b7209abd98e6d3efbd30e02 ]]
            version=4.2.0
            bin=MonitorControl
            requirement='notarized and identifier "me.guillaumeb.MonitorControl" and certificate leaf[subject.OU] = "CYC8C8R4K9"'
            ;;
    esac
    archive="$profile/tmp/$filename"
    curl --proto '=https' --proto-redir '=https' --tlsv1.2 --location -fsS --max-time 240 --max-filesize "$size" "$url" -o "$archive"
    [[ "$(stat -f%z "$archive")" == "$size" ]]
    [[ "$(shasum -a 256 "$archive" | cut -d ' ' -f 1)" == "$digest" ]]
    policy="(version 1) (allow default) (deny network*) (deny file-write*) (allow file-write* (subpath \"$profile/Applications\"))"
    if [[ "$id" == monitorcontrol ]]; then
        fixture_mount="$profile/migration-mount"
        mkdir "$fixture_mount"
        trap 'hdiutil detach "$fixture_mount" >/dev/null 2>&1 || true; if [[ "$profile" == "$root/profile-$id."* ]] && test -f "$profile/.shift-overnight-cycle"; then /bin/rm -rf -- "$profile"; fi' EXIT
        hdiutil attach -readonly -nobrowse -noautoopen -verify -mountpoint "$fixture_mount" "$archive" >/dev/null
        test -d "$fixture_mount/$app.app" && test ! -L "$fixture_mount/$app.app"
        sandbox-exec -p "$policy" /usr/bin/ditto --rsrc --extattr --noacl "$fixture_mount/$app.app" "$profile/Applications/$app.app"
        hdiutil detach "$fixture_mount" >/dev/null
    else
        sandbox-exec -p "$policy" /usr/bin/ditto -x -k "$archive" "$profile/Applications"
    fi
    fixture="$profile/Applications/$app.app"
    [[ "$(plutil -extract CFBundleShortVersionString raw "$fixture/Contents/Info.plist")" == "$version" ]]
    /usr/bin/codesign --verify --all-architectures --deep --strict "$fixture"
    /usr/bin/codesign --verify --check-notarization -R="$requirement" "$fixture"
    /usr/bin/lipo "$fixture/Contents/MacOS/$bin" -verify_arch arm64
    /bin/rm -- "$archive"
    printf 'PASS SIGNED MIGRATION FIXTURE %s %s\n' "$app" "$old_tag"
    phases=(update-from-fixture remove)
fi
for phase in "${phases[@]}"; do
    run_phase "$phase"
    [[ "$(cat "$profile/synthetic-user-data.txt")" == "synthetic user data" ]]
    if [[ "$id" == wwdc && "$phase" != remove ]]; then
        run_phase open
        python3 - "$profile/Applications/WWDC.app/Contents/MacOS/WWDC" <<'PYWWDC'
import os, signal, subprocess, sys, time

executable = sys.argv[1]

def current_processes():
    processes = subprocess.check_output(['/bin/ps', '-axww', '-o', 'pid=,comm='], text=True)
    return {int(pid) for line in processes.splitlines() if line.strip()
            for pid, command in [line.strip().split(maxsplit=1)] if command == executable}

for attempt in range(50):
    matches = current_processes()
    if len(matches) == 1:
        break
    time.sleep(0.2)
else:
    raise SystemExit('WWDC did not start exactly one process from the isolated bundle')

[pid] = matches
time.sleep(2)
assert pid in current_processes(), 'WWDC exited immediately'
os.kill(pid, signal.SIGTERM)
for attempt in range(50):
    if pid not in current_processes():
        break
    time.sleep(0.2)
else:
    raise SystemExit('WWDC did not exit after SIGTERM')
print('PASS OPEN AND CLOSE WWDC executable in the isolated profile')
PYWWDC
    fi

    if [[ ( "$id" == openttd || "$id" == blender || "$id" == gimp || "$id" == vlc || "$id" == inks || "$id" == scummvm ) && "$phase" != remove ]]; then
        python3 - "$id" "$profile" "$phase" "$old" "$new" <<'PYLAUNCH'
from pathlib import Path
import re, subprocess, sys
app_id, profile, phase, old, new = sys.argv[1:]
version = old if phase == 'install' else new
bundle, binary, args = {
    'openttd': ('OpenTTD', 'openttd', ['-h']),
    'scummvm': ('ScummVM', 'scummvm', ['--version']),
    'blender': ('Blender', 'Blender', ['--version']),
    'gimp': ('GIMP', 'gimp-3.2', ['--version']),
    'vlc': ('VLC', 'VLC', ['-I', 'dummy', '--no-plugins-cache', '--version']),
    'inks': ('Inkscape', 'inkscape', ['--version']),
}[app_id]
executable = Path(profile) / f'Applications/{bundle}.app/Contents/MacOS/{binary}'
command = ['/usr/bin/sandbox-exec', '-p', '(version 1) (allow default) (deny network*) (deny file-write*)', str(executable), *args]
env = {'HOME': profile, 'CFFIXED_USER_HOME': profile, 'TMPDIR': f'{profile}/tmp', 'PATH': '/usr/bin:/bin'}
output = subprocess.run(command, env=env, check=True, capture_output=True, text=True, timeout=15).stdout
lines = output.splitlines()
version_line = next((line for line in lines if line.startswith('ScummVM ')), '') if app_id == 'scummvm' else lines[0]
assert re.search(rf'\b{re.escape(version.removeprefix("v"))}\b', version_line), output[:160]
command_label = ' '.join(args)
print(f'PASS CLI {binary} {command_label}: {version_line}')
PYLAUNCH
    fi
done
test -d "$profile/.Trash/$app.app"
printf 'PASS NATIVE CYCLE %s (%s → %s), private Trash and synthetic data preserved\n' "$id" "$old" "$new"
