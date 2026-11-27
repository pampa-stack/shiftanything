#!/usr/bin/env bash
set -euo pipefail
root=$(realpath -e -- "$1")
app=${2:?}
phase=${3:?}
[[ "$root" == /var/tmp/shift-flatpak-desktop.* ]]
case "$app" in cherrytree|foliate|pdf4qt|gnome-calendar) ;; *) exit 2 ;; esac
# Flatpak isolates the app; the private session bus and display are created only for this run.
unset SSH_AUTH_SOCK SSH_AGENT_PID
export XDG_RUNTIME_DIR="$root/$app/home/runtime"
exec /usr/bin/dbus-run-session -- bash "$root/run-daily.sh" "$root" open "$app" "$phase"
