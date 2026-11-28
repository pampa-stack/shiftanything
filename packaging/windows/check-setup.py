#!/usr/bin/env python3
"""Check that a file is the ShiftAnything NSIS setup for one version, before its updater signature."""
import json
import sys
from pathlib import Path

# The Windows product name has one source: the Tauri Windows config.
WINDOWS_CONFIG = Path(__file__).resolve().parents[2] / "apps/shiftanything-tauri-proof/tauri.windows.json"

NSIS_SIGNATURE = b"\xef\xbe\xad\xdeNullsoftInst"


def string_values(data: bytes, key: str) -> set[str]:
    """Every value of a VS_VERSIONINFO string entry, read from the PE version resource."""
    marker = key.encode("utf-16-le") + b"\0\0"
    values = set()
    start = data.find(marker)
    while start != -1:
        at = start + len(marker)
        at += -at % 4
        end = at
        while end + 1 < len(data) and data[end : end + 2] != b"\0\0":
            end += 2
        values.add(data[at:end].decode("utf-16-le", "replace"))
        start = data.find(marker, start + 1)
    return values


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: check-setup.py SETUP VERSION", file=sys.stderr)
        return 2
    data = Path(sys.argv[1]).read_bytes()
    version = sys.argv[2]
    if not data.startswith(b"MZ") or NSIS_SIGNATURE not in data:
        print("error: not an NSIS installer", file=sys.stderr)
        return 1
    product = json.loads(WINDOWS_CONFIG.read_text())["productName"]
    name, declared = string_values(data, "ProductName"), string_values(data, "ProductVersion")
    if name != {product} or declared != {version}:
        print(f"error: setup declares {sorted(name)} {sorted(declared)}, expected {product} {version}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
