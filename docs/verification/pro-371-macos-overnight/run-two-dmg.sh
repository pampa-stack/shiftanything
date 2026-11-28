#!/bin/bash
set -euo pipefail
umask 077
root="$(cd "$(dirname "$0")" && pwd -P)"
id=$1
case "$id" in
    playcover)
        repo=PlayCover/PlayCover app=PlayCover bundle_id=io.playcover.PlayCover team=792V9HMJW3
        ;;
    material-maker)
        repo=RodZill4/material-maker app='Material Maker' bundle_id=com.rodzlabs.materialmaker team=79HXNZ6Y77
        ;;
    mullvad-browser)
        repo=mullvad/mullvad-browser app='Mullvad Browser' bundle_id=net.mullvad.mullvadbrowser team=MADPSAYN6T
        ;;
    openboard)
        repo=OpenBoard-org/OpenBoard app=OpenBoard bundle_id=org.oe-f.OpenBoard team=A6HNXYT969
        ;;
    markedit)
        repo=MarkEdit-app/MarkEdit app=MarkEdit bundle_id=app.cyan.markedit team=TCKG8FBVG6
        ;;
    capso)
        repo=lzhgus/Capso app=Capso bundle_id=com.awesomemacapps.capso team=T247WTJV77
        ;;
    vorssaint)
        repo=vorssaint/vorssaint-utils app=Vorssaint bundle_id=com.vorssaint.utils team=3D485NHW29
        ;;
    onionshare)
        repo=onionshare/onionshare app=OnionShare bundle_id=OnionShare team=G762K6CH36
        ;;
    aiderdesk)
        repo=magiksoft/aider-desk app=aider-desk bundle_id=eu.magiksoft.aiderdesk team=S585J6UY5L
        ;;
    zulip)
        repo=zulip/zulip-desktop app=Zulip bundle_id=org.zulip.zulip-electron team=66KHCWMEYB
        ;;
    *) exit 2 ;;
esac
[[ $(uname -m) == arm64 && "$root" == /private/tmp/shift-air-overnight.* && -O "$root" && -f "$root/.shift-owned" ]]
profile=$(mktemp -d "$root/profile-$id.XXXXXXXX")
touch "$profile/.shift-overnight-cycle"
cleanup() {
    if [[ "$profile" == "$root/profile-$id."* && -f "$profile/.shift-overnight-cycle" ]]; then
        find "$profile" -xdev -depth -delete
    fi
}
trap cleanup EXIT
printf 'synthetic user data\n' > "$profile/synthetic-user-data.txt"
mkdir -p "$profile/"{tmp,Applications,.Trash,Library/Caches/ShiftAnything}
cd "$root"
python3 - "$repo" "$id.tsv" "$profile/Library/Caches/ShiftAnything/releases.json" <<'PY'
from pathlib import Path
import json, sys, time
repo, tsv, cache = sys.argv[1:]
releases = []
for line in Path(tsv).read_text().splitlines():
    tag, name, size, digest, url = line.split('\t')
    version = tag.removeprefix('v')
    if version.count('.') == 1:
        version += '.0'
    releases.append(dict(tag=tag, version=version, name=tag, published_at=None,
                         html_url=f'https://github.com/{repo}/releases/tag/{tag}', notes='',
                         prerelease=False, assets=[dict(name=name, size=int(size), sha256=digest, url=url)]))
Path(cache).write_text(json.dumps({'entries': {}, 'versions': {repo: [int(time.time()), releases[::-1]]}}) + '\n')
PY
sw_vers -productVersion
spctl --status
run_phase() {
    env -i HOME="$profile" CFFIXED_USER_HOME="$profile" TMPDIR="$profile/tmp" PATH=/usr/bin:/bin:/usr/sbin:/sbin "$root/native-cycle" "$id" "$1" "$app"
}
bundle="$profile/Applications/$app.app"
for phase in install update; do
    run_phase "$phase"
    executable=$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$bundle/Contents/Info.plist")
    /usr/bin/lipo "$bundle/Contents/MacOS/$executable" -verify_arch arm64
    /usr/bin/codesign --verify --all-architectures --deep --strict "$bundle"
    /usr/bin/codesign --verify --check-notarization -R="notarized and identifier \"$bundle_id\" and certificate leaf[subject.OU] = \"$team\"" "$bundle"
    /usr/sbin/spctl -a -vv -t execute "$bundle"
    /usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$bundle/Contents/Info.plist"
    printf 'PASS ARCHITECTURE SIGNATURE GATEKEEPER %s\n' "$phase"
done
run_phase remove
test -d "$profile/.Trash/$app.app"
test "$(cat "$profile/synthetic-user-data.txt")" = 'synthetic user data'
printf 'PASS NATIVE CYCLE %s, private Trash and synthetic data preserved\n' "$id"
