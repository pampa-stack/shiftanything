#!/usr/bin/env bash
set -euo pipefail
umask 077
[[ $(hostname) == shift-launcher-qa && $(id -un) == qa && $(dpkg --print-architecture) == amd64 ]]
phase=${1:?}; profile=${2:?}
[[ $profile == /home/qa/n.* && -d $profile && -O $profile && ! -L $profile ]]
export HOME="$profile" FLATPAK_USER_DIR="$profile/flatpak" FLATPAK_SYSTEM_DIR="$profile/flatpak-system"
export XDG_DATA_HOME="$profile/data" XDG_CACHE_HOME="$profile/cache" XDG_CONFIG_HOME="$profile/config" XDG_STATE_HOME="$profile/state"
export XDG_RUNTIME_DIR="$profile/run" TMPDIR="$profile/tmp" XAUTHORITY="$profile/run/xauth" LIBGL_ALWAYS_SOFTWARE=1
export XDG_CURRENT_DESKTOP=XFCE XDG_SESSION_TYPE=x11
ref=com.naps2.Naps2
read_version() { flatpak list --user --app --columns=application,version | awk -v id="$ref" '$1 == id {print $2}'; }
check_exports() {
    local exports="$FLATPAK_USER_DIR/exports/share"
    test -f "$exports/applications/$ref.desktop"
    grep -q '^Name=NAPS2$' "$exports/applications/$ref.desktop"
    find -L "$exports/icons" -type f -name "$ref.*" | awk 'NR==1 {print} END { exit NR == 0 }'
}
probe() { timeout 260 "$profile/naps2-native-cycle" "$1" 2>&1 | tee "$profile/$1.log"; }
if [[ $phase == prepare ]]; then
    (( $(df -B1 --output=avail "$profile" | tail -1) > 11*1024**3 ))
    mkdir -p "$XDG_DATA_HOME" "$XDG_CACHE_HOME" "$XDG_CONFIG_HOME" "$XDG_STATE_HOME" "$XDG_RUNTIME_DIR" "$TMPDIR" "$FLATPAK_USER_DIR" "$FLATPAK_SYSTEM_DIR/repo"
    : > "$profile/.naps2-proof"
    printf 'synthetic history data\n' > "$profile/marker.txt"
    ostree init --repo="$FLATPAK_SYSTEM_DIR/repo" --mode=bare-user-only
    flatpak remote-add --user --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
    flatpak install --user --noninteractive -y flathub org.freedesktop.Platform/x86_64/25.08
    flatpak info --user --show-commit org.freedesktop.Platform/x86_64/25.08
    exit
fi
test -f "$profile/.naps2-proof"
if [[ $phase == cycle ]]; then
    if flatpak info --user "$ref" >/dev/null 2>&1 || flatpak info --system "$ref" >/dev/null 2>&1 || dpkg-query -W -f='${Status}\n' naps2 2>/dev/null | grep -qx 'install ok installed'; then
        echo 'NAPS2 already present in dedicated guest' >&2; exit 1
    fi
    # Invoked by the EXIT trap below.
    # shellcheck disable=SC2329
    cleanup() {
        flatpak kill "$ref" 2>/dev/null || true
        flatpak uninstall --user --noninteractive -y "$ref" 2>/dev/null || true
    }
    trap cleanup EXIT
    offline() {
        sudo -n unshare --net --pid --fork --kill-child=TERM --mount-proc bash -eu -c '
            ip link set dev lo up
            exec setpriv --reuid "$1" --regid "$2" --clear-groups --inh-caps=-all --ambient-caps=-all                 env HOME=/home/qa USER=qa LOGNAME=qa LANG=C.UTF-8 dbus-run-session -- bash "$3" "$4" "$5"
        ' _ "$(id -u)" "$(id -g)" "$profile/run.sh" "$1" "$profile"
    }
    probe install
    [[ $(read_version) == 8.6.0 ]]
    check_exports
    offline open-old
    marker=".var/app/$ref/data/launcher-synthetic.txt"
    mkdir -p "$profile/${marker%/*}"
    cp "$profile/marker.txt" "$profile/$marker"
    probe update
    [[ $(read_version) == 8.6.1 ]]
    check_exports
    cmp "$profile/marker.txt" "$profile/$marker"
    offline open-new
    probe remove
    [[ -z $(read_version) ]]
    test ! -e "$FLATPAK_USER_DIR/exports/share/applications/$ref.desktop"
    icons="$FLATPAK_USER_DIR/exports/share/icons"
    if [[ -d "$icons" ]]; then
        remaining=$(find -L "$icons" -type f -name "$ref.*")
        [[ -z $remaining ]]
    fi
    cmp "$profile/marker.txt" "$profile/$marker"
    printf 'PASS user install, two GUI opens, update, menu/icon export, removal and app data preservation\n'
    exit
fi
[[ $phase == open-old || $phase == open-new ]]
[[ $(awk -F: 'NR>2 {gsub(/ /,"",$1); print $1}' /proc/net/dev) == lo ]]
cat /proc/net/dev > "$profile/$phase.network"
export DISPLAY=:123
: > "$XAUTHORITY"
xauth -f "$XAUTHORITY" add "$DISPLAY" . "$(mcookie)"
Xvfb "$DISPLAY" -auth "$XAUTHORITY" -screen 0 1280x800x24 -nolisten tcp > "$profile/$phase.xvfb" 2>&1 &
xpid=$!
wm=
finish_gui() {
    flatpak kill "$ref" 2>/dev/null || true
    [[ -z $wm ]] || kill "$wm" 2>/dev/null || true
    kill "$xpid" 2>/dev/null || true
    wait "$xpid" 2>/dev/null || true
}
trap finish_gui EXIT
for ((n=0;n<35;n++)); do xdpyinfo >/dev/null 2>&1 && break; sleep .2; done
xdpyinfo >/dev/null
dbus-update-activation-environment DISPLAY XAUTHORITY XDG_RUNTIME_DIR XDG_CURRENT_DESKTOP XDG_SESSION_TYPE HOME XDG_DATA_HOME XDG_CONFIG_HOME XDG_CACHE_HOME
xfwm4 --compositor=on --display="$DISPLAY" > "$profile/$phase.wm" 2>&1 &
wm=$!
probe "$phase"
window=$(timeout 35 xdotool search --sync --onlyvisible --name 'NAPS2' | sed -n '1p')
xprop -id "$window" WM_CLASS WM_NAME | tee "$profile/$phase.window"
sleep 4
xwininfo -id "$window" -stats | grep 'Map State: IsViewable'
flatpak ps --columns=application,pid | grep "^$ref"
xwd -silent -root -out "$profile/$phase.xwd"
