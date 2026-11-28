#!/usr/bin/env bash
set -euo pipefail
umask 077
[[ $(hostname) == shift-launcher-qa && $(id -un) == qa && $(dpkg --print-architecture) == amd64 ]]
slug=${1:?candidate}
mode=${2:-all}
case "$slug" in openscreen) title="[Oo]pen[Ss]creen"; min_width=600 min_height=150; display=118 ;; neovide) title="[Nn]eovide"; min_width=500 min_height=400; display=119 ;; *) exit 2 ;; esac
base=/home/qa
bin="$base/native-appimages-next237"
[[ -x $bin ]]
out="$base/catalog-evidence/$slug-native-appimage-next237"
export PATH="/home/qa/qa-neovim/nvim-linux-x86_64/bin:/usr/bin:/bin"
export HOME="$base/profile-$slug-native-appimage-next237"
export XDG_DATA_HOME="$HOME/data" XDG_CONFIG_HOME="$HOME/config" XDG_CACHE_HOME="$HOME/cache" XDG_STATE_HOME="$HOME/state"
if [[ $mode == open-new ]]; then display=$((display + 3)); fi
export XDG_RUNTIME_DIR="$HOME/runtime" TMPDIR="$HOME/tmp" DISPLAY=:$display XAUTHORITY="$HOME/runtime/xauth" LIBGL_ALWAYS_SOFTWARE=1
probe() { timeout 270 "$bin" "$slug" "$1" 2>&1 | tee "$out/$1.log"; }
state() { printf '%s\t%s\n' "$(date -Is)" "$*" | tee -a "$out/stages.tsv"; }
offline() {
    timeout 115 sudo -n unshare --net --pid --fork --kill-child=TERM --mount-proc \
        bash -eu -c 'ip link set dev lo up; exec setpriv --reuid "$1" --regid "$2" --clear-groups --inh-caps=-all --ambient-caps=-all env HOME=/home/qa USER=qa LOGNAME=qa LANG=C.UTF-8 dbus-run-session -- bash /home/qa/native-appimages-guest-next237.sh "${@:3}"' \
        _ "$(id -u)" "$(id -g)" "$slug" "$1"
}
finish() {
    offline open-new
    probe remove
    grep -qx 'Synthetic data should survive' "$HOME/synthetic.txt"
    state 'PASS NATIVE APPIMAGE PACKAGE AND GUI CYCLE; DATA PRESERVED'
}
if [[ $mode == all ]]; then
    mkdir "$out" "$HOME"
    mkdir -m700 "$XDG_DATA_HOME" "$XDG_CONFIG_HOME" "$XDG_CACHE_HOME" "$XDG_STATE_HOME" "$XDG_RUNTIME_DIR" "$TMPDIR" "$HOME/Applications"
    : > "$HOME/.shift-native-appimage-proof"
    avail=$(df -B1 --output=avail "$HOME" | tail -1)
    ((avail > 11*1024**3))
    state "DISK AVAILABLE $avail"
    stat -c %a "$HOME/.shift-native-appimage-proof" | grep -x 600
    printf 'Synthetic data should survive\n' > "$HOME/synthetic.txt"
    ldd "$bin" > "$out/linked-libraries.txt"
    dpkg-query -W libfuse2t64 > "$out/fuse-runtime.txt"
    probe install
    cp "$XDG_DATA_HOME"/applications/*.desktop "$out/previous.desktop"
    offline open-old
    probe update
    if [[ $slug == openscreen ]]; then
        test ! -e "$HOME/Applications/Openscreen-Linux-2.3.0.AppImage"
    fi
    cp "$XDG_DATA_HOME"/applications/*.desktop "$out/current.desktop"
    finish
    exit 0
fi
[[ $mode == open-old || $mode == open-new ]]
test "$(tail -n +3 /proc/net/dev | cut -d: -f1 | tr -d " \t")" = lo
cat /proc/net/dev > "$out/$mode-network.txt"
: > "$XAUTHORITY"
xauth -f "$XAUTHORITY" add "$DISPLAY" . "$(mcookie)"
Xvfb "$DISPLAY" -auth "$XAUTHORITY" -screen 0 1280x900x24 -nolisten tcp > "$out/$mode-xvfb.log" 2>&1 & xpid=$!
wpid=
cleanup() {
    [[ -z $wpid ]] || kill "$wpid" 2>/dev/null || true
    kill "$xpid" 2>/dev/null || true
}
trap cleanup EXIT
for ((n=0;n<45;n++)); do xdpyinfo >/dev/null 2>&1 && break; sleep .2; done
xdpyinfo >/dev/null
export XDG_CURRENT_DESKTOP=XFCE XDG_SESSION_TYPE=x11
dbus-update-activation-environment DISPLAY XAUTHORITY XDG_RUNTIME_DIR XDG_CURRENT_DESKTOP XDG_SESSION_TYPE HOME XDG_DATA_HOME XDG_CONFIG_HOME XDG_CACHE_HOME
if [[ "$mode" == open-old ]]; then
    python3 -c 'import pathlib,secrets,sys; x=secrets.token_hex(32); pathlib.Path(sys.argv[1]).write_text(x)' "$HOME/runtime/keyring-password"
fi
[[ $(stat -c %a "$HOME/runtime/keyring-password") == 600 ]]
gnome-keyring-daemon --unlock < "$HOME/runtime/keyring-password" > "$out/$mode-keyring.log" 2>&1
gdbus call --session --dest org.freedesktop.secrets --object-path /org/freedesktop/secrets     --method org.freedesktop.Secret.Service.SetAlias default /org/freedesktop/secrets/collection/login >/dev/null
gdbus call --session --dest org.freedesktop.secrets --object-path /org/freedesktop/secrets/collection/login     --method org.freedesktop.DBus.Properties.Get org.freedesktop.Secret.Collection Locked | grep -F '<false>' > "$out/$mode-keyring-unlocked.log"
xfwm4 --compositor=off --display="$DISPLAY" > "$out/$mode-wm.log" 2>&1 & wpid=$!
probe "$mode"
window=
for ((n=0;n<90;n++)); do
    case "$slug" in
        neovide) candidate=$(xdotool search --onlyvisible --class "$title" 2>/dev/null | tail -1) || candidate= ;;
        openscreen) candidate=$(xdotool search --onlyvisible --name "$title" 2>/dev/null | tail -1) || candidate= ;;
    esac
    if [[ -n $candidate ]] && xdotool getwindowname "$candidate" > "$out/$mode-window.txt" 2>/dev/null &&
        xdotool getwindowgeometry "$candidate" >> "$out/$mode-window.txt" 2>/dev/null &&
        awk -F '[ x]+' -v w="$min_width" -v h="$min_height" '$2 == "Geometry:" && $3 >= w && $4 >= h { seen=1 } END { exit !seen }' "$out/$mode-window.txt"; then
        window=$candidate
        break
    fi
    sleep .5
done
sleep 15
xwininfo -root -tree > "$out/$mode-window-tree.txt"
xwd -silent -root -out "$out/$mode.xwd"
[[ -n $window ]]
xprop -id "$window" WM_CLASS >> "$out/$mode-window.txt"
state "GUI OFFLINE $mode"
xdotool windowclose "$window" || true
if [[ $mode == open-new ]]; then rm -- "$HOME/runtime/keyring-password"; fi
