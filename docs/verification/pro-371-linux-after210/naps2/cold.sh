#!/usr/bin/env bash
set -euo pipefail
umask 077
[[ $(hostname) == shift-launcher-qa && $(id -un) == qa ]]
probe=$1
[[ $probe == /home/qa/n.*/naps2-native-cycle && -f $probe && -O $probe ]]
profile=$(mktemp -d /home/qa/n.XXXXXXXX)
printf 'Cold profile %s\n' "$profile"
export HOME="$profile" FLATPAK_USER_DIR="$profile/flatpak" FLATPAK_SYSTEM_DIR="$profile/flatpak-system"
export XDG_DATA_HOME="$profile/data" XDG_CACHE_HOME="$profile/cache" XDG_CONFIG_HOME="$profile/config" XDG_STATE_HOME="$profile/state"
export XDG_RUNTIME_DIR="$profile/run" TMPDIR="$profile/tmp"
mkdir -p "$XDG_DATA_HOME" "$XDG_CACHE_HOME" "$XDG_CONFIG_HOME" "$XDG_STATE_HOME" "$XDG_RUNTIME_DIR" "$TMPDIR" "$FLATPAK_USER_DIR" "$FLATPAK_SYSTEM_DIR/repo"
touch "$profile/.naps2-proof"
ostree init --repo="$FLATPAK_SYSTEM_DIR/repo" --mode=bare-user-only
echo 'Remotes before:'
flatpak remotes --user --columns=name,url
timeout 260 "$probe" install || result=$?
echo 'Remotes after:'
flatpak remotes --user --columns=name,url
echo 'Runtimes and apps:'
flatpak list --user --columns=ref,version,installation
if flatpak info --user com.naps2.Naps2 >/dev/null 2>&1; then
    flatpak uninstall --user -y --noninteractive com.naps2.Naps2
fi
exit "${result:-0}"
