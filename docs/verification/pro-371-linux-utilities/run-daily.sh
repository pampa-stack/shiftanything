#!/usr/bin/env bash
set -euo pipefail
root=$(realpath -e -- "$1")
[[ "$root" == /var/tmp/shift-flatpak-desktop.* ]]
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
  cherrytree) id=net.giuspen.cherrytree; title="cherrytree|cherry.?tree"; old=146db7290d2b169b72cd27a1da7faa690bb02247e6dc244e9dc8b118ff3f0be6; expected=306d32be9bdb60fea1a0a37e3aa36008690aebf634d84522924f0b82fb0d2f57 ;;
  foliate) id=com.github.johnfactotum.Foliate; title="foliate"; old=f0ccefbb5b7b52e8e1ad7ab51db8ecc3e229a540cdfef8d2bb903762393630e6; expected=4ca532b545422340d60651e8d4a912192dade5dc6f0af264f051215c5e6f489c ;;
  pdf4qt) id=io.github.JakubMelka.Pdf4qt; title="pdf4qt|pdf4qteditor|pdf editor"; old=2e1b6551b978e4feb96fd54d4127615e730ff6d51407122f0fde63485c2d6ddd; expected=74aa672411f5f82032b7bdbc5a3e771b15616de215d16ecd54f050dbe513eca1 ;;
  gnome-calendar) id=org.gnome.Calendar; title="gnome.calendar|calendar"; old=f2e677fbdcc96a412ac5b2d0054efc60c1e654079a7bd242d20b65b5ae306826; expected=54b27758494108a51e1747428909fe8337d21ddb923894b2ff3418b881de738a ;;
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
icon() {
    local location source icon_path
    location=$(flatpak info --user --show-location "$ref")
    for subpath in 128x128/apps/$id.png scalable/apps/$id.svg 256x256/apps/$id.png 64x64/apps/$id.png; do
        for root_icons in "$location/export/share/icons/hicolor" "$location/files/share/icons/hicolor"; do
            source="$root_icons/$subpath"
            if [[ -f "$source" ]]; then
                icon_path="$root/evidence/native-$app-icon.${source##*.}"
                cp "$source" "$icon_path"
                file "$icon_path"
                sha256sum "$icon_path"
                printf '%s\n' "$subpath" > "$root/evidence/native-$app-icon-source.txt"
                return
            fi
        done
    done
    printf '%s\n' "No icon found in installed Flatpak: $id" >&2
    return 1
}
case "$mode" in
  install)
    existing_remote=$(flatpak remotes --user --show-disabled --columns=name:full)
    test -z "$existing_remote" || test "$existing_remote" = flathub
    "$native" install "$app"
    remote=$(flatpak remotes --user --show-disabled --columns=name:full)
    test "$remote" = flathub
    flatpak info --user --show-commit "$ref" > "$root/evidence/native-$app-current-commit"
    test "$(cat "$root/evidence/native-$app-current-commit")" = "$expected"
    verify_commit "$(cat "$root/evidence/native-$app-current-commit")" current
    icon
    flatpak list --user --app --columns=ref:full,origin:full,active:full,installation:full,version:full
    ;;
  prior)
    current=$(cat "$root/evidence/native-$app-current-commit")
    test "$current" != "$old"
    prior_runtime=$(flatpak remote-info --user --commit="$old" --show-runtime flathub "$ref")
    [[ "$prior_runtime" =~ ^org\.(gnome|kde|freedesktop)\.Platform/x86_64/[[:digit:].-]+$ ]]
    if ! flatpak info --user "runtime/$prior_runtime" >/dev/null 2>&1; then
      flatpak install --user --assumeyes --noninteractive flathub "runtime/$prior_runtime"
    fi
    printf 'Previous runtime %s\n' "$prior_runtime"
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
    marker="$HOME/.var/app/$id/shift-profile-marker"
    printf 'user profile data\n' > "$marker"
    sha256sum "$marker" > "$root/evidence/native-$app-marker.sha256"
    ;;
  open)
    phase=$4
    flags=(--unshare=network --nofilesystem=host:reset --nosocket=system-bus --nosocket=session-bus --nosocket=wayland --socket=x11 --nodevice=all --nosocket=pulseaudio --nofilesystem=xdg-run/pipewire-0)
    flatpak override --user "${flags[@]}" --env=GSK_RENDERER=cairo --env=LIBGL_ALWAYS_SOFTWARE=1 --env=GDK_BACKEND=x11 --env=QT_QPA_PLATFORM=xcb --env=LP_NUM_THREADS=2 "$id"
    evidence="$root/evidence/native-$app-$phase"
    mkdir -p "$FLATPAK_SYSTEM_DIR"
    test -f "$FLATPAK_SYSTEM_DIR/repo/config" || ostree --repo="$FLATPAK_SYSTEM_DIR/repo" init --mode=bare-user-only
    export SHIFT_APP_OUTPUT="$evidence.app.log" PATH="$root/bin:$PATH"
    display=$((300 + BASHPID % 500))
    export DISPLAY=:$display XAUTHORITY="$XDG_RUNTIME_DIR/native-xauth-$app-$phase"
    touch "$XAUTHORITY"
    xauth -f "$XAUTHORITY" add "$DISPLAY" . "$(mcookie)"
    Xvfb "$DISPLAY" -auth "$XAUTHORITY" -screen 0 1440x1080x24 -nolisten tcp > "$evidence.xvfb.log" 2>&1 &
    xpid=$!
    FLATPAK="$root/bin/flatpak-portal-child" \
      "$SHIFT_TEST_SYSROOT/usr/libexec/flatpak-portal" --verbose > "$evidence.portal.log" 2>&1 &
    portal_pid=$!
    clean() {
      flatpak kill "$id" >/dev/null 2>&1 || :
      kill "$xpid" 2>/dev/null || :
      kill "$portal_pid" 2>/dev/null || :
      wait "$xpid" "$portal_pid" 2>/dev/null || :
    }
    trap clean EXIT
    for tries in {1..25}; do if xdpyinfo > /dev/null 2>&1; then break; fi; sleep 0.2; done
    for tries in {1..20}; do
      if gdbus call --session --dest org.freedesktop.portal.Flatpak --object-path /org/freedesktop/portal/Flatpak \
          --method org.freedesktop.DBus.Peer.Ping > /dev/null 2>&1; then break; fi
      test "$tries" -lt 20
      sleep .2
    done
    flatpak run --user --command=sh "$ref" -c 'readlink /proc/self/ns/net; cat /proc/net/dev' > "$evidence.network.log"
    test "$(head -1 "$evidence.network.log")" != "$(readlink /proc/self/ns/net)"
    test "$(awk -F: 'NR>3{gsub(/ /,"",$1); print $1}' "$evidence.network.log")" = lo
    # Start each app on the same synthetic background, without a personal desktop.
    xsetroot -solid '#25415a'
    open_start=$(date +%s%3N)
    "$native" open "$app"
    window=
    for tries in {1..220}; do
      xwininfo -root -tree > "$evidence.windows.txt"
      candidates=$(awk -v name="$title" '
        tolower($0) !~ /crash|debug-tool| splash/ && tolower($0) ~ name && match($0, /[0-9]+x[0-9]+[+-]/) {
          split(substr($0, RSTART, RLENGTH - 1), size, "x");
          if (size[1] >= 200 && size[2] >= 200) print size[1] * size[2], $1;
        }
      ' "$evidence.windows.txt" | sort -k1,1nr | cut -d' ' -f2)
      while read -r candidate; do
        if [[ -n "$candidate" ]] &&
           xwininfo -id "$candidate" -stats | grep -q 'Map State: IsViewable' &&
           ! xprop -id "$candidate" _NET_WM_WINDOW_TYPE | grep -q '_NET_WM_WINDOW_TYPE_SPLASH'; then
          sleep 1
          if xwininfo -id "$candidate" -stats | grep -q "Map State: IsViewable"; then
            window="$candidate"
            break
          fi
        fi
      done <<< "$candidates"
      if [[ -n "$window" ]]; then break; fi
      sleep .4
    done
    test -n "$window"
    xwininfo -id "$window" -stats > "$evidence.window.txt"
    grep -q 'Map State: IsViewable' "$evidence.window.txt"
    xprop -id "$window" WM_NAME WM_CLASS > "$evidence.properties.txt"
    xwd -silent -id "$window" -out "$evidence.xwd"
    echo "TIMING $app open-$phase $(( $(date +%s%3N) - open_start )) ms"
    if [[ -n "$window" ]]; then
        xdotool windowfocus "$window" key --clearmodifiers ctrl+q || :
        for tries in {1..12}; do
            xwininfo -id "$window" >/dev/null 2>&1 || break
            sleep .25
        done
    fi
    ;;
  update-remove)
    trap 'flatpak kill "$id" >/dev/null 2>&1 || :' EXIT
    update_start=$(date +%s%3N)
    "$native" update "$app"
    echo "TIMING $app update $(( $(date +%s%3N) - update_start )) ms"
    test "$(flatpak info --user --show-commit "$ref")" = "$(cat "$root/evidence/native-$app-current-commit")"
    sha256sum -c "$root/evidence/native-$app-marker.sha256"
    bash "$root/protected-open.sh" "$root" "$app" current
    bash "$root/run-daily.sh" "$root" remove "$app"
    ;;
  remove)
    remove_start=$(date +%s%3N)
    "$native" remove "$app"
    echo "TIMING $app remove $(( $(date +%s%3N) - remove_start )) ms"
    if flatpak info --user "$ref" >/dev/null 2>&1; then exit 4; fi
    sha256sum -c "$root/evidence/native-$app-marker.sha256"
    printf 'PASS native install, open, update, remove, persisted ownership and profile data\n'
    ;;
esac
