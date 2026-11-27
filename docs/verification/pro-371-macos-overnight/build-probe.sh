#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
root="$(pwd -P)"
test -f .shift-owned
python3 - "$root" <<'PY'
from pathlib import Path
import json, subprocess, sys
root = Path(sys.argv[1])
artifacts = {}
success = False
for line in (root / 'recipe-build.jsonl').read_text().splitlines():
    event = json.loads(line)
    if event["reason"] == "compiler-artifact" and event["target"]["name"] in ('artcraft_launcher_core', 'artcraft_launcher_native'):
        [filename] = [path for path in event["filenames"] if path.endswith('.rlib')]
        artifacts[event["target"]["name"]] = filename
    if event["reason"] == "build-finished":
        success = event["success"]
assert success
args = [str(Path.home() / '.cargo/bin/rustc'), '--edition=2024', '-L', f'dependency={root}/target/debug/deps']
for name, filename in artifacts.items():
    args += ['--extern', f'{name}={filename}']
subprocess.run([*args, 'native-cycle.rs', '-o', 'native-cycle'], check=True)
PY
