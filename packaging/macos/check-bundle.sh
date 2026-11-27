#!/usr/bin/env bash
# Checks shared by the local and Developer ID disk images.

check_bundle_modes() {
  local bundle="$1" path
  for path in "$bundle" "$bundle/Contents" "$bundle/Contents/MacOS" "$bundle/Contents/Resources" \
    "$bundle/Contents/_CodeSignature" "$bundle/Contents/MacOS/ShiftAnything"; do
    [[ "$(stat -f '%Lp' "$path")" = 755 ]] || { printf 'Expected mode 755: %s\n' "$path" >&2; return 1; }
  done
  for path in "$bundle/Contents/Info.plist" "$bundle/Contents/_CodeSignature/CodeResources"; do
    [[ "$(stat -f '%Lp' "$path")" = 644 ]] || { printf 'Expected mode 644: %s\n' "$path" >&2; return 1; }
  done
}

check_bundle_version() {
  local bundle="$1" version="$2" actual
  actual="$("$bundle/Contents/MacOS/ShiftAnything" --version)" || return 1
  [[ "$actual" = "shiftanything $version" ]] || { printf 'Unexpected launcher version: %s\n' "$actual" >&2; return 1; }
  printf '%s\n' "$actual"
}

stage_dmg_layout() {
  local stage="$1" layout="$2"
  mkdir -m 755 "$stage/.background"
  install -m 644 "$layout/background.tiff" "$stage/.background/background.tiff"
  install -m 644 "$layout/dmg-layout.DS_Store" "$stage/.DS_Store"
  ln -s /Applications "$stage/Applications"
}

check_dmg_layout() (
  local mount="$1" layout="$2" entry
  shopt -s nullglob dotglob
  for entry in "$mount"/*; do
    case "${entry##*/}" in
      'Sh*ftAnything.app'|Applications|.DS_Store|.background) ;;
      *) printf 'Unexpected item in the macOS installer: %s\n' "${entry##*/}" >&2; return 1 ;;
    esac
  done
  if [[ ! -d "$mount/Sh*ftAnything.app" || -L "$mount/Sh*ftAnything.app" ||
        ! -L "$mount/Applications" || "$(readlink "$mount/Applications")" != /Applications ||
        ! -f "$mount/.DS_Store" || ! -f "$mount/.background/background.tiff" ]] ||
     ! cmp -s "$layout/dmg-layout.DS_Store" "$mount/.DS_Store" ||
     ! cmp -s "$layout/background.tiff" "$mount/.background/background.tiff"; then
    printf 'Missing or invalid macOS installer layout.\n' >&2
    return 1
  fi
)
