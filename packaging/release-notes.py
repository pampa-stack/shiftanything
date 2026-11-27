#!/usr/bin/env python3
"""Generate GitHub release notes from the source used to build both packages."""

import argparse
import json
from pathlib import Path
import re
import subprocess


ROOT = Path(__file__).resolve().parents[1]
TAURI = "apps/shiftanything-tauri-proof"
TARGETS = (
    ("Mac con chip Apple", "macos", "arm64"),
    ("Linux x64", "linux", "x64"),
)
MIN_NEW_VERIFICATIONS = 20


def read_json(commit, filename):
    return json.loads(
        subprocess.check_output(
            ["git", "show", f"{commit}:{filename}"], cwd=ROOT, text=True
        )
    )


def resolve_commit(ref):
    return subprocess.check_output(
        ["git", "rev-parse", "--verify", "--end-of-options", f"{ref}^{{commit}}"],
        cwd=ROOT,
        text=True,
    ).strip()


def catalog_counts(commit):
    catalog = read_json(commit, "crates/core/curated.json")
    ids = {app["id"] for app in catalog}
    if not ids or len(ids) != len(catalog):
        raise ValueError("expected a nonempty catalog with unique app IDs")

    verified = read_json(commit, f"{TAURI}/verified-platforms.json")
    counts = {}
    for label, platform, arch in TARGETS:
        entries = verified[platform][arch]
        unique_ids = set(entries)
        if len(unique_ids) != len(entries) or not unique_ids <= ids:
            raise ValueError(f"{label}: duplicate or unknown app IDs in verification evidence")
        counts[label] = unique_ids
    return len(ids), counts


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("ref", help="Commit or tag used to build the release")
    parser.add_argument("--previous", help="Previous public release commit or tag to enforce the catalog targets")
    parser.add_argument("--changes", type=Path, help="Markdown highlights for this release")
    args = parser.parse_args()

    try:
        commit = resolve_commit(args.ref)
        catalog_size, counts = catalog_counts(commit)
        if args.previous:
            previous = resolve_commit(args.previous)
            _, baseline = catalog_counts(previous)
            deltas = {label: len(ids - baseline[label]) for label, ids in counts.items()}
            total = sum(deltas.values())
            if total < MIN_NEW_VERIFICATIONS:
                progress = ", ".join(f"{label} +{delta}" for label, delta in deltas.items())
                raise ValueError(f"catalog release target not reached ({total}/{MIN_NEW_VERIFICATIONS}; {progress})")
        version = read_json(commit, f"{TAURI}/tauri.conf.json")["version"]
    except (ValueError, subprocess.CalledProcessError) as error:
        parser.error(str(error))

    if not isinstance(version, str) or not re.fullmatch(r"\d+\.\d+\.\d+", version):
        parser.error("expected a three-part version from tauri.conf.json")

    print(f"## Sh*ftAnything {version} beta\n")
    if args.changes:
        print(args.changes.read_text(encoding="utf-8").strip(), end="\n\n")
    print(f"### Catalogo · {catalog_size} schede\n")
    for label, ids in counts.items():
        print(f"- **{label}** — {len(ids)} app con ciclo di pacchetto verificato.")
    print("\nUsa **Installabili qui** per vedere quali app puoi scaricare sul tuo computer.")
    print(f"\nCodice usato per la build: `{commit}`.")


if __name__ == "__main__":
    main()
