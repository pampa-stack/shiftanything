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
