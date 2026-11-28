#!/usr/bin/env python3
"""Generate GitHub release notes from the source used to build both packages."""

import argparse
import json
from pathlib import Path
import re
import subprocess


ROOT = Path(__file__).resolve().parents[1]
TAURI = "apps/shiftanything-tauri-proof"


def read_json(commit, filename):
    return json.loads(
        subprocess.check_output(
            ["git", "show", f"{commit}:{filename}"], cwd=ROOT, text=True
        )
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("ref", help="Commit or tag used to build the release")
    parser.add_argument("--changes", type=Path, help="Markdown highlights for this release")
    args = parser.parse_args()

    commit = subprocess.check_output(
        ["git", "rev-parse", "--verify", "--end-of-options", f"{args.ref}^{{commit}}"],
        cwd=ROOT,
        text=True,
    ).strip()
    version = read_json(commit, f"{TAURI}/tauri.conf.json")["version"]
    if not isinstance(version, str) or not re.fullmatch(r"\d+\.\d+\.\d+", version):
        parser.error("expected a three-part version from tauri.conf.json")
    catalog = read_json(commit, "crates/core/curated.json")
    ids = {app["id"] for app in catalog}
    if not ids or len(ids) != len(catalog):
        parser.error("expected a nonempty catalog with unique app IDs")

    verified = read_json(commit, f"{TAURI}/verified-platforms.json")
    targets = {"Mac con chip Apple": verified["macos"]["arm64"], "Linux x64": verified["linux"]["x64"]}
    for label, entries in targets.items():
        if len(set(entries)) != len(entries) or not set(entries) <= ids:
            parser.error(f"{label}: duplicate or unknown app IDs in verification evidence")

    print(f"## Sh*ftAnything {version} beta\n")
    if args.changes:
        print(args.changes.read_text(encoding="utf-8").strip(), end="\n\n")
    print(f"### Catalogo · {len(ids)} schede\n")
    for label, entries in targets.items():
        print(f"- **{label}** — {len(entries)} app con ciclo di pacchetto verificato.")
    print("\nUsa **Installabili qui** per vedere quali app puoi scaricare sul tuo computer.")
    print(f"\nCodice usato per la build: `{commit}`.")


if __name__ == "__main__":
    main()
