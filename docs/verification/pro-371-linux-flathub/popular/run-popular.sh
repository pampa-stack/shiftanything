#!/usr/bin/env bash
set -euo pipefail
root=$1
mode=$2
source /var/tmp/shift-tauri-flathub.ysriyaXg/env.sh
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$SHIFT_TEST_SYSROOT/usr/lib/x86_64-linux-gnu/libproxy"
app=$3
profile="$root/$app/home"
export HOME=$profile XDG_DATA_HOME=$profile/data XDG_CONFIG_HOME=$profile/config
export XDG_CACHE_HOME=$profile/cache XDG_STATE_HOME=$profile/state XDG_RUNTIME_DIR=$profile/runtime
export FLATPAK_USER_DIR=$profile/data/flatpak FLATPAK_SYSTEM_DIR=$profile/system-flatpak
export FLATPAK_CONFIG_DIR=$profile/config/flatpak-system
export GSK_RENDERER=cairo LIBGL_ALWAYS_SOFTWARE=1 GDK_BACKEND=x11 LP_NUM_THREADS=2
case "$app" in
  krita) id=org.kde.krita; title=krita; old=59fe7ecf2377eee71b154bd29096704c2800f56026dcf0bbf54e0e694854d70d ;;
  obs) id=com.obsproject.Studio; title=obs; old=1e330b0905f47f5f32725844404da917f008a0d3b9fa6ebf30813860d718a0d6 ;;
  celluloid) id=io.github.celluloid_player.Celluloid; title=celluloid; old=299765ac29b8ad0dfbf4aca214a249eb93ab78423a02f45c6d11dd71f5b20152 ;;
  handbrake) id=fr.handbrake.ghb; title=handbrake; old=52ab1ebd671627776be790f2c73702c3e62e60e58e01f0fff0b4605370e7de6a ;;
  *) exit 2 ;;
esac
ref=app/$id/x86_64/stable
native=$root/native-cycle
umask 077
mkdir -p "$profile"/{data,config,cache,state,runtime}
verify_commit() {
    local output="$root/evidence/native-$app-$2.gpg.txt"
    ostree --repo="$FLATPAK_USER_DIR/repo" show --gpg-verify-remote=flathub "$1" > "$output"
    grep -F 'Good signature' "$output"
}
case "$mode" in
  install)
    existing_remote=$(flatpak remotes --user --show-disabled --columns=name:full)
    test -z "$existing_remote" || test "$existing_remote" = flathub
    "$native" install "$app"
    remote=$(flatpak remotes --user --show-disabled --columns=name:full)
    test "$remote" = flathub
    flatpak info --user --show-commit "$ref" > "$root/evidence/native-$app-current-commit"
    verify_commit "$(cat "$root/evidence/native-$app-current-commit")" current
    flatpak list --user --app --columns=ref:full,origin:full,active:full,installation:full,version:full
    ;;
  prior)
    current=$(cat "$root/evidence/native-$app-current-commit")
    test "$current" != "$old"
    flatpak update --user --assumeyes --noninteractive --commit="$old" "$ref"
    test "$(flatpak info --user --show-commit "$ref")" = "$old"
    verify_commit "$old" previous
    flatpak list --user --app --columns=ref:full,origin:full,active:full,installation:full,version:full
    "$native" external "$app"
    flatpak_location=$(flatpak info --user --show-location "$ref")
    manifest=$(find "$XDG_DATA_HOME" -name installed.json -print -quit)
    test -f "$manifest"
    cp -- "$manifest" "$root/evidence/native-$app-manifest-before-fixture.json"
    python3 - "$manifest" "$flatpak_location" "$old" "$app" <<'PY'
import json, os, pathlib, sys
manifest, app_location, commit, app_id = sys.argv[1:]
path = pathlib.Path(manifest)
state = json.loads(path.read_text())
record = state['flathub'][app_id]
info = os.stat(pathlib.Path(app_location) / 'deploy')
record.update(commit=commit, device=info.st_dev, inode=info.st_ino,
              modified_seconds=info.st_mtime_ns // 1_000_000_000,
              modified_nanos=info.st_mtime_ns % 1_000_000_000)
path.write_text(json.dumps(state) + '\n')
PY
    mkdir -p "$HOME/.var/app/$id"
    printf 'user profile data\n' > "$HOME/.var/app/$id/shift-profile-marker"
    ;;
  open)
    phase=$4
    flags=(--unshare=network --nofilesystem=host:reset --nosocket=system-bus --nosocket=session-bus --nosocket=wayland --socket=x11 --nodevice=all --nosocket=pulseaudio --nofilesystem=xdg-run/pipewire-0)
    flatpak override --user "${flags[@]}" --env=GSK_RENDERER=cairo --env=LIBGL_ALWAYS_SOFTWARE=1 --env=GDK_BACKEND=x11 --env=QT_QPA_PLATFORM=xcb --env=LP_NUM_THREADS=2 "$id"
    evidence="$root/evidence/native-$app-$phase"
    display=$((300 + BASHPID % 500))
    export DISPLAY=:$display XAUTHORITY="$XDG_RUNTIME_DIR/native-xauth-$app-$phase"
    touch "$XAUTHORITY"
    xauth -f "$XAUTHORITY" add "$DISPLAY" . "$(mcookie)"
    Xvfb "$DISPLAY" -auth "$XAUTHORITY" -screen 0 1440x1080x24 -nolisten tcp > "$evidence.xvfb.log" 2>&1 &
    xpid=$!
    clean() {
      flatpak kill "$id" >/dev/null 2>&1 || :
      kill "$xpid" 2>/dev/null || :
      wait "$xpid" 2>/dev/null || :
    }
    trap clean EXIT
    for tries in {1..25}; do if xdpyinfo > /dev/null 2>&1; then break; fi; sleep 0.2; done
    flatpak run --user --command=sh "$ref" -c 'readlink /proc/self/ns/net; cat /proc/net/dev' > "$evidence.network.log"
    test "$(head -1 "$evidence.network.log")" != "$(readlink /proc/self/ns/net)"
    test "$(awk -F: 'NR>3{gsub(/ /,"",$1); print $1}' "$evidence.network.log")" = lo
    "$native" open "$app"
    window=
    for tries in {1..75}; do
      xwininfo -root -tree > "$evidence.windows.txt"
      candidates=$(awk -v name="$title" '
        tolower($0) ~ name && match($0, /[0-9]+x[0-9]+[+-]/) {
          split(substr($0, RSTART, RLENGTH - 1), size, "x");
          if (size[1] >= 200 && size[2] >= 200) print size[1] * size[2], $1;
        }
      ' "$evidence.windows.txt" | sort -k1,1nr | cut -d' ' -f2)
      while read -r candidate; do
        if [[ -n "$candidate" ]] &&
           xwininfo -id "$candidate" -stats | grep -q 'Map State: IsViewable' &&
           ! xprop -id "$candidate" _NET_WM_WINDOW_TYPE | grep -q '_NET_WM_WINDOW_TYPE_SPLASH'; then
          window="$candidate"
          break
        fi
      done <<< "$candidates"
      if [[ -n "$window" ]]; then break; fi
      sleep .4
    done
    test -n "$window"
    sleep 3
    xwininfo -id "$window" -stats > "$evidence.window.txt"
    grep -q 'Map State: IsViewable' "$evidence.window.txt"
    xprop -id "$window" WM_NAME WM_CLASS > "$evidence.properties.txt"
    xwd -silent -id "$window" -out "$evidence.xwd"
    if [[ "$app" == obs ]]; then
        xdotool windowfocus "$window" key --clearmodifiers ctrl+q || :
        for tries in {1..12}; do
            xwininfo -id "$window" >/dev/null 2>&1 || break
            sleep .25
        done
    fi
    ;;
  update-remove)
    trap 'flatpak kill "$id" >/dev/null 2>&1 || :' EXIT
    "$native" update "$app"
    test "$(flatpak info --user --show-commit "$ref")" = "$(cat "$root/evidence/native-$app-current-commit")"
    grep -qx 'user profile data' "$HOME/.var/app/$id/shift-profile-marker"
    dbus-run-session -- bash "$root/run-popular.sh" "$root" open "$app" current
    bash "$root/run-popular.sh" "$root" remove "$app"
    ;;
  remove)
    "$native" remove "$app"
    if flatpak info --user "$ref" >/dev/null 2>&1; then exit 4; fi
    grep -qx 'user profile data' "$HOME/.var/app/$id/shift-profile-marker"
    printf 'PASS native install, open, update, remove, persisted ownership and profile data\n'
    ;;
esac
