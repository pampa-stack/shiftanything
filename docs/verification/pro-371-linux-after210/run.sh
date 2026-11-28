#!/usr/bin/env bash
set -euo pipefail
umask 077

slug=${1:?Catalog ID}
display_number=${2:?Dedicated X server number}
[[ $(hostname) == shift-launcher-qa && $(id -un) == qa && $display_number =~ ^[0-9]{2}$ ]]
guest_home=$HOME
compositor=off
profile=$slug-after210
if [[ ${SHIFT_QA_COMPOSITOR:-off} == on ]]; then
    compositor=on
    profile=$slug-composited
fi
out=$guest_home/evidence/$profile
read -r flatid pattern < <(awk -v slug="$slug" '$1==slug{print $2,$3}' "$guest_home/after210-candidates")
previous=$(awk -v slug="$slug" '$1==slug{print $4}' "$guest_home/after210-candidates")
current=$(awk -v slug="$slug" '$1==slug{print $5}' "$guest_home/after210-candidates")
[[ $flatid =~ ^[a-zA-Z0-9._-]+$ && $previous =~ ^[0-9a-f]{64}$ && $current =~ ^[0-9a-f]{64}$ && $previous != "$current" ]]
if [[ "$slug" == frog && "$flatid" == com.github.tenderowl.frog ]]; then
    # Under X11 both the window name and class are set by the Python launcher.
    pattern='"python":[[:space:]]+[(]"python"[[:space:]]+"python"[)]'
fi
ref=app/$flatid/x86_64/stable
native=$guest_home/flatpak-after210-cycle
export HOME=$guest_home/profiles/$profile
mkdir -p "$out" "$HOME"/{data,cache,config,state,runtime,system-flatpak}
export XDG_DATA_HOME=$HOME/data XDG_CONFIG_HOME=$HOME/config
export XDG_CACHE_HOME=$HOME/cache XDG_STATE_HOME=$HOME/state XDG_RUNTIME_DIR=$HOME/runtime
export FLATPAK_USER_DIR=$XDG_DATA_HOME/flatpak FLATPAK_CONFIG_DIR=$XDG_CONFIG_HOME/flatpak-system
export FLATPAK_SYSTEM_DIR=$HOME/system-flatpak
export DISPLAY=:$display_number XAUTHORITY=$XDG_RUNTIME_DIR/xauth
if [[ "$slug" == easyeffects ]]; then
    # Flatpak exports xdg-run from the canonical runtime directory in this guest.
    XDG_RUNTIME_DIR=/run/user/$(id -u)
    export XDG_RUNTIME_DIR
    test -O "$XDG_RUNTIME_DIR" && test "$(stat -c %a "$XDG_RUNTIME_DIR")" = 700
fi
export GDK_BACKEND=x11 QT_QPA_PLATFORM=xcb QT_QUICK_BACKEND=software
export GSK_RENDERER=cairo LIBGL_ALWAYS_SOFTWARE=1 LP_NUM_THREADS=2
ostree --repo="$FLATPAK_SYSTEM_DIR/repo" init --mode=bare-user-only

say() { printf '%s %s %s\n' "$(date -Is)" "$slug" "$*"; }
disk_guard() {
    local remaining
    remaining=$(df -B1 --output=avail "$HOME" | tail -1)
    say "AVAILABLE $remaining"
    test "$remaining" -gt "$((9 * 1024**3))"
}
stage=setup
declare -a services=()
cleanup() {
    local result=$?
    set +e
    flatpak kill "$flatid" >/dev/null 2>&1
    for pid in "${services[@]}"; do kill "$pid" 2>/dev/null; done
    if flatpak info --user "$ref" >/dev/null 2>&1; then
        flatpak uninstall --user -y --noninteractive "$ref" >> "$out/cleanup.log" 2>&1
    fi
    flatpak uninstall --user -y --noninteractive --unused >> "$out/cleanup.log" 2>&1
    flatpak list --user --columns=ref:full,version > "$out/remaining.txt"
    say "EXIT $result $stage"
    exit "$result"
}
trap cleanup EXIT

verify() {
    local hash=$1 phase=$2
    test "$(flatpak info --user --show-commit "$ref")" = "$hash"
    ostree --repo="$FLATPAK_USER_DIR/repo" show --gpg-verify-remote=flathub "$hash" > "$out/$phase.signature.txt"
    grep -F 'Good signature' "$out/$phase.signature.txt"
    grep -F '4184DD4D907A7CAE' "$out/$phase.signature.txt"
    flatpak info --user "$ref" > "$out/$phase.package.txt"
    say "$phase $hash"
}

open_gui() {
    local phase=$1 window='' candidate verify_window='' dialog width height
    if [[ "$slug" == easyeffects ]]; then
        # shellcheck disable=SC2016
        flatpak run --user --command=sh "$ref" -c '
            test -S "$XDG_RUNTIME_DIR/$PIPEWIRE_REMOTE" || { echo "Missing private PipeWire socket"; exit 1; }
            test ! -e "$XDG_RUNTIME_DIR/pipewire-0" || { echo "Unexpected default PipeWire socket"; exit 1; }
            test ! -e /dev/snd || { echo "Unexpected ALSA device directory"; exit 1; }
        '
        say "$phase sandbox sees the private PipeWire socket and no /dev/snd"
    fi
    flatpak run --user --command=sh "$ref" -c 'readlink /proc/self/ns/net; cat /proc/net/dev' > "$out/$phase.network.txt"
    test "$(head -1 "$out/$phase.network.txt")" != "$(readlink /proc/self/ns/net)"
    test "$(awk -F: 'NR>3 {gsub(/ /,"",$1); print $1}' "$out/$phase.network.txt")" = lo
    if ! "$native" open "$slug"; then
        timeout 8 flatpak run --user --env=QT_DEBUG_PLUGINS=1 "$ref" > "$out/$phase.launch-debug.txt" 2>&1 || true
        return 1
    fi
    sleep 8
    case "$slug" in
        calibre)
            if [[ "$phase" == previous ]] && dialog=$(timeout 20 xdotool search --sync --onlyvisible --name '^calibre Welcome wizard$' | sed -n '1p'); then
                xdotool windowactivate --sync "$dialog" key alt+n
                sleep 1
                xdotool windowactivate --sync "$dialog" key alt+n
                sleep 1
                xdotool windowactivate --sync "$dialog" key alt+f
            fi
            ;;
        ardour)
            say "On $DISPLAY, create the Shift-QA session ($phase) and start the private PulseAudio output"
            timeout 120 xdotool search --sync --onlyvisible --name "$pattern" >/dev/null
            ;;
        zim)
            if [[ "$phase" == previous ]]; then
                dialog=$(timeout 20 xdotool search --sync --onlyvisible --name '^Add Notebook$' | sed -n '1p')
                xdotool windowactivate --sync "$dialog" key Tab ctrl+a
                xdotool type "$HOME/Notes"
                read -r width height < <(xdotool getwindowgeometry --shell "$dialog" | awk -F= '
                    $1 == "WIDTH" {width = $2}
                    $1 == "HEIGHT" {print width, $2}
                ')
                xdotool mousemove --window "$dialog" "$((width - 55))" "$((height - 29))" click 1
                test -d "$HOME/Notes"
            fi
            ;;
        fontforge)
            dialog=$(timeout 20 xdotool search --sync --onlyvisible --name '^Open Font$' | sed -n '1p')
            read -r width height < <(xdotool getwindowgeometry --shell "$dialog" | awk -F= '
                $1 == "WIDTH" {width = $2}
                $1 == "HEIGHT" {print width, $2}
            ')
            # The English Open Font dialog places New between OK and Cancel.
            xdotool windowactivate --sync "$dialog"
            xdotool mousemove --window "$dialog" "$((width / 2))" "$((height - 20))" click 1
            timeout 20 xdotool search --sync --onlyvisible --name '^Untitled' >/dev/null
            ;;
        freeplane)
            if dialog=$(timeout 20 xdotool search --sync --onlyvisible --name '^Select template$' | sed -n '1p'); then
                xdotool windowactivate --sync "$dialog" key Return
                sleep 4
            fi
            ;;
        opentoonz)
            dialog=$(timeout 35 xdotool search --sync --onlyvisible --name '^OpenToonz Startup$' | sed -n '1p')
            xdotool windowactivate --sync "$dialog" key Alt+F4
            for ((attempt=0;attempt<25;attempt++)); do
                if ! xwininfo -id "$dialog" 2>/dev/null | grep -Fq 'Map State: IsViewable'; then break; fi
                sleep .4
            done
            if xwininfo -id "$dialog" 2>/dev/null | grep -Fq 'Map State: IsViewable'; then return 1; fi
            ;;
    esac
    for ((attempt=0;attempt<110;attempt++)); do
        xwininfo -root -tree > "$out/$phase.window-tree.txt"
        while read -r candidate; do
            if xwininfo -id "$candidate" -stats 2>/dev/null | grep 'Map State: IsViewable' >/dev/null; then
                if [[ $verify_window == "$candidate" ]]; then
                    window=$candidate
                    break
                fi
                verify_window=$candidate
                break
            fi
        done < <(awk -v name="$pattern" '
            tolower($0) ~ tolower(name) && match($0, /[0-9]+x[0-9]+[+-]/) {
                split(substr($0, RSTART, RLENGTH - 1), size, "x")
                if (size[1] >= 200 && size[2] >= 100) print size[1] * size[2], $1
            }
        ' "$out/$phase.window-tree.txt" | sort -rn | cut -d' ' -f2)
        [[ -z $window ]] || break
        sleep .4
    done
    if [[ -z "$window" ]]; then
        xwininfo -root -tree > "$out/$phase.no-window.txt"
        xwd -silent -root -out "$out/$phase.no-window.xwd"
        return 1
    fi
    xwininfo -id "$window" -stats > "$out/$phase.window.txt"
    grep -F 'Map State: IsViewable' "$out/$phase.window.txt"
    xprop -id "$window" WM_NAME WM_CLASS > "$out/$phase.properties.txt"
    flatpak ps --columns=application | grep -Fx "$flatid"
    xwd -silent -root -out "$out/$phase.xwd" || cp "$out/framebuffer/Xvfb_screen0" "$out/$phase.xwd"
    xdotool windowactivate --sync "$window" key Alt+F4 || true
    for ((attempt=0;attempt<20;attempt++)); do
        if ! flatpak ps --columns=application | grep -Fxq "$flatid"; then break; fi
        sleep .3
    done
    if flatpak ps --columns=application | grep -Fxq "$flatid"; then flatpak kill "$flatid"; fi
    for ((attempt=0;attempt<10;attempt++)); do
        if ! flatpak ps --columns=application | grep -Fxq "$flatid"; then break; fi
        sleep .3
    done
    if flatpak ps --columns=application | grep -Fxq "$flatid"; then return 1; fi
    say "$phase GUI $(cat "$out/$phase.properties.txt")"
}

disk_guard
stage=install
"$native" install "$slug"
verify "$current" current-install
location=$(flatpak info --user --show-location "$ref")
find "$location/export/share" -type f \( -name "$flatid.desktop" -o -name "$flatid.png" -o -name "$flatid.svg" \) | sort > "$out/menu-exports.txt"
grep -F "$flatid.desktop" "$out/menu-exports.txt"
grep -E '[.](png|svg)$' "$out/menu-exports.txt"
stage=external-prior
historical_runtime=$(flatpak remote-info --user --commit="$previous" --show-runtime flathub "$ref")
[[ $historical_runtime =~ ^org\.(gnome|kde|freedesktop)\.(Platform|Sdk)/x86_64/[0-9.-]+$ ]]
if ! flatpak info --user "runtime/$historical_runtime" >/dev/null 2>&1; then
    flatpak install --user -y --noninteractive flathub "runtime/$historical_runtime"
fi
flatpak update --user -y --noninteractive --commit="$previous" "$ref"
verify "$previous" previous
"$native" external "$slug"
location=$(flatpak info --user --show-location "$ref")
manifest=$(find "$XDG_DATA_HOME" -name installed.json -print -quit)
test -s "$manifest"
python3 - "$manifest" "$location" "$previous" "$slug" <<'PY'
import json, os, pathlib, sys
manifest, location, revision, app = sys.argv[1:]
path = pathlib.Path(manifest)
state = json.loads(path.read_text())
record = state['flathub'][app]
deployment = os.stat(pathlib.Path(location) / 'deploy')
record.update(commit=revision, device=deployment.st_dev, inode=deployment.st_ino,
              modified_seconds=deployment.st_mtime_ns // 1_000_000_000,
              modified_nanos=deployment.st_mtime_ns % 1_000_000_000)
path.write_text(json.dumps(state) + '\n')
PY
mkdir -p "$HOME/.var/app/$flatid"
printf 'Synthetic application data for %s\n' "$flatid" > "$HOME/.var/app/$flatid/fixture.txt"
sha256sum "$HOME/.var/app/$flatid/fixture.txt" > "$out/fixture.sha256"

stage=display
: > "$XAUTHORITY"
xauth -f "$XAUTHORITY" add "$DISPLAY" . "$(mcookie)"
mkdir -p "$out/framebuffer"
Xvfb "$DISPLAY" -fbdir "$out/framebuffer" -auth "$XAUTHORITY" -screen 0 1360x1050x24 -nolisten tcp > "$out/xvfb.log" 2>&1 &
services+=($!)
for ((attempt=0;attempt<30;attempt++)); do if xdpyinfo >/dev/null 2>&1; then break; fi; sleep .2; done
xdpyinfo >/dev/null
xfwm4 --compositor="$compositor" --display="$DISPLAY" > "$out/xfwm4.log" 2>&1 &
services+=($!)
if command -v xsetroot >/dev/null; then xsetroot -solid '#274053'; fi

export XDG_CURRENT_DESKTOP=XFCE
mkdir -p "$XDG_CONFIG_HOME/xdg-desktop-portal"
printf '[preferred]\ndefault=gtk\n' > "$XDG_CONFIG_HOME/xdg-desktop-portal/portals.conf"
dbus-update-activation-environment HOME XDG_DATA_HOME XDG_CONFIG_HOME XDG_CACHE_HOME XDG_RUNTIME_DIR XDG_CURRENT_DESKTOP DISPLAY XAUTHORITY FLATPAK_USER_DIR FLATPAK_CONFIG_DIR FLATPAK_SYSTEM_DIR
flags=(--unshare=network --nofilesystem=host:reset --nosocket=system-bus --nosocket=wayland --nosocket=ssh-auth --socket=x11 --nodevice=all)
flags+=(--no-talk-name=org.freedesktop.Flatpak)
if [[ "$slug" == parabolic ]]; then
    flags+=(--no-talk-name=org.freedesktop.secrets)
fi
if [[ "$slug" == easyeffects ]]; then
    # shellcheck source=/dev/null
    source "$guest_home/easyeffects-pipewire.sh"
    flags+=(--nofilesystem=xdg-run/pipewire-0 "--filesystem=xdg-run/$PIPEWIRE_REMOTE:ro" "--env=PIPEWIRE_REMOTE=$PIPEWIRE_REMOTE")
else
    flags+=(--nofilesystem=xdg-run/pipewire-0)
fi
if [[ "$slug" == ardour || "$slug" == famistudio || "$slug" == hydrogen ]]; then
    # An isolated test signal path, with no hardware and no connection to system audio.
    mkdir -p "$XDG_RUNTIME_DIR/pulse"
    export PULSE_SERVER="unix:$XDG_RUNTIME_DIR/pulse/native"
    pulseaudio -n --daemonize=no --disable-shm=yes --exit-idle-time=-1 --disallow-module-loading=yes \
        --load="module-native-protocol-unix socket=$XDG_RUNTIME_DIR/pulse/native auth-anonymous=1" \
        --load="module-null-sink sink_name=shift_qa" > "$out/pulseaudio.log" 2>&1 &
    services+=($!)
    for ((attempt=0;attempt<30;attempt++)); do
        if pactl info > "$out/pulse-info.txt" 2>/dev/null; then break; fi
        sleep .2
    done
    LC_ALL=C pactl info | grep -Fx 'Default Sink: shift_qa'
    flags+=(--socket=pulseaudio)
else
    flags+=(--nosocket=pulseaudio)
fi
if [[ "$slug" == famistudio ]]; then
    # The disposable guest provides only a virtual ALSA sequencer and null output.
    test -r /dev/snd/seq && test -w /dev/snd/seq
    flags+=(--device=all)
fi
if [[ "$slug" == calibre ]]; then
    calibre_library="$HOME/Calibre Library"
    mkdir -p "$calibre_library"
    flags+=(--filesystem="$calibre_library")
fi
if [[ "$slug" == zim ]]; then
    mkdir -p "$HOME/Notes"
    flags+=(--filesystem="$HOME/Notes")
fi
if [[ "$slug" == bleachbit ]]; then
    mkdir -p "$HOME/.config/bleachbit"
    flags+=(--filesystem="$HOME/.config/bleachbit")
fi
if [[ "$slug" == zed ]]; then
    # Drop test settings left by an earlier run in this isolated profile.
    flatpak override --user --reset "$flatid"
    # Xvfb uses software rendering. Leave the no-escape launch setting to the launcher.
    flags+=(--no-talk-name=org.freedesktop.secrets --nosocket=gpg-agent --env=ZED_ALLOW_EMULATED_GPU=1)
fi
if [[ "$compositor" == on ]]; then
    flags+=(--env=GSK_RENDERER=cairo)
fi
flatpak override --user "${flags[@]}" --env=QT_X11_NO_MITSHM=1 --env=LIBGL_ALWAYS_SOFTWARE=1 --env=LP_NUM_THREADS=2 "$flatid"
flatpak override --user --show "$flatid" > "$out/sandbox.txt"
stage=open-previous
open_gui previous
if [[ "$slug" == calibre ]]; then
    calibre_book=$(find "$calibre_library" -type f -name '*.epub' -print -quit)
    test -s "$calibre_book"
    sha256sum "$calibre_book" > "$out/ebook.sha256"
fi
stage=update
disk_guard
"$native" update "$slug"
verify "$current" current
sha256sum -c "$out/fixture.sha256"
stage=open-current
open_gui current
if [[ "$slug" == calibre ]]; then sha256sum -c "$out/ebook.sha256"; fi
stage=remove
"$native" remove "$slug"
if flatpak info --user "$ref" >/dev/null 2>&1; then exit 1; fi
test ! -e "$FLATPAK_USER_DIR/exports/share/applications/$flatid.desktop"
sha256sum -c "$out/fixture.sha256"
if [[ "$slug" == calibre ]]; then sha256sum -c "$out/ebook.sha256"; fi
say 'PASS signed native cycle, two GUI opens and synthetic data retained'
