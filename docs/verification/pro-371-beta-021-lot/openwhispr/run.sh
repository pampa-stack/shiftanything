#!/bin/bash
set -euo pipefail
umask 077
build=$(cd "$(dirname "$0")" && pwd)
binary=$build/verify
test -x "$binary"
test "$(hostname)" = shift-launcher-qa
base=/var/tmp
id=$1
case "$id" in
    deskreen) repo=pavlobu/deskreen; old=v3.2.15; new=v3.2.16;;
    invoke-community) repo=invoke-ai/launcher; old=v1.8.4; new=v1.8.5;;
    openchamber) repo=openchamber/openchamber; old=v2.8.0; new=v2.8.1;;
    openwork-desktop) repo=different-ai/openwork; old=v0.18.72; new=v0.18.73;;
    recordly) repo=webadderallorg/Recordly; old=v1.3.3; new=v1.4.0;;
    termix) repo=Termix-SSH/Termix; old=v26.11.2; new=v26.11.3;;
    openwhispr) repo=OpenWhispr/openwhispr; old=v1.12.1; new=v1.12.2;;
    *) exit 2;;
esac
df -Pk "$base" | awk 'NR==2 {exit ($4 < 5*1024*1024)}'
SECONDS=0
scratch=$(mktemp -d "$base/shift-official-linux-profile.$id.XXXXXXXX")
proof=$build/proofs/$id
mkdir -p "$proof"
exec > >(tee "$proof/run.log") 2>&1
trap 'status=$?; rm -rf -- "$scratch"; test ! -e "$scratch"; printf "Temporary profile removed; test exit=%s\n" "$status"; exit "$status"' EXIT
printf 'HOST %s\nUTC %s\nPROFILE %s\n' "$(uname -sm)" "$(date -u -Is)" "$scratch"
mkdir "$scratch"/{home,data,config,cache,state,runtime,shots}
chmod 700 "$scratch/runtime"
printf 'synthetic user profile for %s\n' "$id" > "$scratch/config/profile-fingerprint.txt"
touch "$scratch/.shift-official-batch"
cp -- "$binary" "$scratch/verify"
cp -- "$build/$id/"*.json "$build/$id/"*.sha256 "$scratch/"

cat > "$scratch/display.sh" <<'DISPLAY_SESSION'
#!/bin/bash
set -euo pipefail
phase=$1
id=$2
old=$3
new=$4
repo=$5
window_match=${id//-/ }
[[ $id != openwork-desktop ]] || window_match=openwork
# Electron names this window class open-whispr.
[[ $id != openwhispr ]] || window_match=whispr
mkdir -m 1777 /tmp/.X11-unix
export DISPLAY=:88
Xvfb "$DISPLAY" -screen 0 1280x800x24 -nolisten tcp -ac >/tmp/display.log 2>&1 &
xvfb=$!
trap 'kill "$xvfb" 2>/dev/null || true; wait "$xvfb" 2>/dev/null || true' EXIT
for ((i=0;i<35;i++)); do
    if xdotool getdisplaygeometry >/dev/null 2>&1; then break; fi
    sleep .2
done
xdotool getdisplaygeometry
ulimit -c 0
export APPIMAGE_EXTRACT_AND_RUN=1
/fixture/verify "$id" "$phase" "$old" "$new" "$repo"
window=
for ((i=0;i<100;i++)); do
    while read -r wid; do
        title=$(xdotool getwindowname "$wid" 2>/dev/null) || continue
        class=$(xprop -id "$wid" WM_CLASS 2>/dev/null) || continue
        if [[ ${title,,} == *"$window_match"* || ${class,,} == *"$window_match"* ]]; then window=$wid; break; fi
    done < <(xdotool search --onlyvisible --name '.*' 2>/dev/null || true)
    if [[ -n $window ]]; then break; fi
    sleep .25
done
if [[ -n $window ]]; then
    printf 'X11_WINDOW phase=%s title=%s\n' "$phase" "$(xdotool getwindowname "$window")"
    xdotool getwindowgeometry "$window"
    sleep "${SHIFT_QA_SETTLE:-2}"
else
    printf 'NO_APP_WINDOW phase=%s\n' "$phase"
    xwininfo -root -tree || true
fi
python3 - "$phase" <<'PYTHON'
import os, sys
from PIL import ImageGrab

ImageGrab.grab(xdisplay=os.environ["DISPLAY"]).save(f"/fixture/shots/{sys.argv[1]}.png")
PYTHON
test -n "$window" || exit 50
printf "X11_CAPTURED phase=%s\n" "$phase"
DISPLAY_SESSION

sandbox() {
    local mode=$1
    shift
    local extra=()
    case "$mode" in
        download) extra=(--share-net --ro-bind /etc/resolv.conf /etc/resolv.conf --ro-bind /etc/ssl/certs /etc/ssl/certs) ;;
    esac
    timeout --foreground -k 5s 300s bwrap --unshare-all --die-with-parent --new-session --clearenv \
        --ro-bind /usr /usr --symlink usr/bin /bin --symlink usr/lib /lib --symlink usr/lib64 /lib64 \
        --ro-bind /etc/fonts /etc/fonts --ro-bind /etc/passwd /etc/passwd --ro-bind /etc/group /etc/group \
        --proc /proc --dev /dev --tmpfs /tmp --bind "$scratch" /fixture --chdir /fixture \
        --setenv PATH /usr/bin --setenv HOME /fixture/home --setenv USER shift-proof \
        --setenv XDG_CONFIG_HOME /fixture/config --setenv XDG_DATA_HOME /fixture/data \
        --setenv XDG_CACHE_HOME /fixture/cache --setenv XDG_STATE_HOME /fixture/state \
        --setenv XDG_RUNTIME_DIR /fixture/runtime --setenv TMPDIR /tmp --setenv LC_ALL C.UTF-8 \
        --setenv LP_NUM_THREADS 2 --setenv LIBGL_ALWAYS_SOFTWARE 1 --setenv GDK_BACKEND x11 --setenv SHIFT_QA_SETTLE "${SHIFT_QA_SETTLE:-2}" \
        --hostname shift-appimage-fixture "${extra[@]}" -- "$@"
}

snapshot_prefs() {
    (cd "$scratch"; find config data -type f ! -path 'data/shiftanything/*' ! -path 'data/applications/*' ! -path 'data/icons/*' ! -path 'data/Trash/*' -print0 | sort -z | xargs -0r sha256sum)
}

show_app() {
    local phase=$1 status=0
    sandbox display /usr/bin/dbus-run-session -- /usr/bin/bash /fixture/display.sh "$phase" "$id" "$old" "$new" "$repo" || status=$?
    if [[ -s $scratch/shots/$phase.png ]]; then
        cp -- "$scratch/shots/$phase.png" "$proof/$phase.png"
    fi
    return "$status"
}
sandbox download /fixture/verify "$id" install "$old" "$new" "$repo"
mapfile -t entries < <(find "$scratch/data/applications" -name "*.desktop" -type f)
(( ${#entries[@]} == 1 ))
desktop=${entries[0]}
test -s "$desktop"
grep '^Exec=' "$desktop" | tee "$proof/desktop-install.txt"
gui_old=0
show_app launch-old || gui_old=$?
snapshot_prefs | tee "$proof/profile-before-update.txt"
test -s "$proof/profile-before-update.txt"
sandbox download /fixture/verify "$id" update "$old" "$new" "$repo"
snapshot_prefs | tee "$proof/profile-after-update.txt"
cmp "$proof/profile-before-update.txt" "$proof/profile-after-update.txt"
grep '^Exec=' "$desktop" | tee "$proof/desktop-update.txt"
gui_new=0
show_app launch || gui_new=$?
snapshot_prefs | tee "$proof/profile-before-uninstall.txt"
sandbox isolated /fixture/verify "$id" uninstall "$old" "$new" "$repo"
test ! -e "$desktop"
snapshot_prefs | tee "$proof/profile-after-uninstall.txt"
cmp "$proof/profile-before-uninstall.txt" "$proof/profile-after-uninstall.txt"
printf 'PASS %s install, update, uninstall, profile preserved; GUI old exit=%s, new exit=%s\n' "$id" "$gui_old" "$gui_new"
printf "Elapsed seconds: %s\n" "$SECONDS"
(( gui_old == 0 && gui_new == 0 ))
