# shellcheck shell=bash
# Upstream-only release environment. Intentionally blocked in the Sh*ftAnything fork.
# No output folders, signing material or publishing settings are initialized.
upstream_packaging_disabled() {
  echo 'Upstream release packaging is disabled in Sh*ftAnything. Use packaging/macos/local-dmg.sh for an ad-hoc local test.' >&2
  return 1
}
if ! upstream_packaging_disabled; then
  exit 1
fi
