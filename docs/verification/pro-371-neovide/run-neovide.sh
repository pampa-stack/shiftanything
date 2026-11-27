#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
root=$(pwd -P)
[[ -f "$root/.shift-mac-probe" ]]
test "$(uname -m)" = arm64
home="$root/home-neovide"
umask 077
mkdir -p "$home"/{Apps,.Trash,tmp,config,cache,data}
if [[ ! -e "$home/data-to-preserve.txt" ]]; then
  printf 'fixture data\n' >"$home/data-to-preserve.txt"
fi
bound=/Users/pampa
profile="$home/cli-only.sb"
cat >"$profile" <<EOF
(version 1)
(allow default)
(deny network*)
(deny mach-lookup)
(deny file-read* (subpath "$bound"))
(deny file-write*)
(allow file-write* (subpath "$home") (literal "/dev/null"))
EOF
begin_iso() {
  df -Pk /private/tmp "$root"
  [[ $(df -Pk /private/tmp | awk 'END{print $4}') -ge 750000 ]]
  [[ $(df -Pk "$root" | awk 'END{print $4}') -ge 550000 ]]
}
phase() {
  begin_iso
  env -i HOME="$home" CFFIXED_USER_HOME="$home" TMPDIR=/private/tmp PATH=/usr/bin:/bin:/usr/sbin:/sbin \
    "$root/cycle" neovide Neovide neovide/neovide com.neovide.neovide X8CNW77992 "$@"
}
check_isolation() {
  local guard
  guard=$(/usr/bin/mktemp -d /private/tmp/shift-cli-canary.XXXXXXXX)
  local rc=0
  /usr/bin/sandbox-exec -f "$profile" /bin/sh -c 'printf denied > "$1/should-not-exist"' _ "$guard" || rc=$?
  if [[ "$rc" = 0 || -e "$guard/should-not-exist" ]]; then
    printf 'Sandbox canary was permitted\n'
    return 1
  fi
  /bin/rmdir "$guard"
  printf 'PASS sandbox denied writes outside proof home\n'
}
run_version() {
  local version=$1
  local executable="$home/Apps/Neovide.app/Contents/MacOS/neovide"
  /usr/bin/codesign --verify --deep --strict "$executable"
  /usr/bin/lipo -archs "$executable"
  /usr/bin/time -p /usr/bin/env -i HOME="$home" CFFIXED_USER_HOME="$home" TMPDIR="$home/tmp" XDG_CACHE_HOME="$home/cache" \
    XDG_CONFIG_HOME="$home/config" XDG_DATA_HOME="$home/data" PATH=/usr/bin:/bin:/usr/sbin:/sbin \
    /usr/bin/sandbox-exec -f "$profile" "$executable" --version | tee "$home/version-$version.log"
  rg -qx "neovide $version" "$home/version-$version.log"
  printf 'PASS restricted process --version %s\n' "$version"
}
check_isolation
phase install 0.17.3 Neovide-aarch64-apple-darwin.dmg 12337549 024f11f8e5a69b58cd9f9c01cf5d6bbe5c4813474f0c5cc1150cddb70e7cd2cc
run_version 0.17.3
phase update 0.17.4 Neovide-aarch64-apple-darwin.dmg 12338261 5a4647cce0703552906552ae1d0d955e221f3153af832fa6739ffc0592774e2a
run_version 0.17.4
phase remove
