"""Ordina per stelle GitHub le app desktop senza un ciclo verificato."""

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TARGETS = ("macos-arm64", "linux-x64", "windows-x64")


def read_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def plan(apps, evidence, platform, arch, target):
    catalog_ids = [app["id"] for app in apps]
    if len(set(catalog_ids)) != len(catalog_ids):
        raise ValueError("ID duplicati nel catalogo")
    if platform not in evidence or arch not in evidence[platform]:
        raise ValueError(f"Evidenza di ciclo assente per {platform}-{arch}")

    desktop = [app for app in apps if app["delivery"] == "desktop"]
    desktop_ids = {app["id"] for app in desktop}
    recorded = set(evidence[platform][arch])
    verified = recorded & desktop_ids
    incompatible = []
    candidates = []
    for app in desktop:
        app_id = app["id"]
        if app_id in verified:
            continue
        release = app.get("release") or {}
        release_package = any(pkg["platform"] == platform for pkg in release.get("packages", []))
        channels = [channel for channel in app.get("distribution", []) if channel["platform"] == platform]
        platform_source = app.get("platform_support", {}).get(platform)
        if app.get("platform_scope_complete") and not (release_package or channels or platform_source):
            incompatible.append(app_id)
            continue
        package_sources = [release["source"]] if release_package else []
        package_sources.extend(channel["source"] for channel in channels)
        stars = app.get("stars") or {}
        candidates.append({
            "id": app_id,
            "github_stars": stars.get("count"),
            "stars_date": stars.get("date"),
            "package_sources": sorted(set(package_sources)),
            "platform_source": platform_source,
        })

    # Treat missing stars separately from zero and break ties by ID.
    candidates.sort(key=lambda row: (
        row["github_stars"] is None, -(row["github_stars"] or 0), row["id"]
    ))
    with_package = []
    without_package = []
    for rank, row in enumerate(candidates, start=1):
        row["rank"] = rank
        group = with_package if row["package_sources"] else without_package
        group.append(row)

    return {
        "platform": platform,
        "architecture": arch,
        "goal": target,
        "catalog_entries": len(apps),
        "desktop_entries": len(desktop),
        "non_desktop_entries": len(apps) - len(desktop),
        "verified_cycles": len(verified),
        "cycles_remaining": max(0, target - len(verified)),
        "verified_ids_outside_desktop_catalog": sorted(recorded - desktop_ids),
        "documented_incompatible_count": len(incompatible),
        "documented_incompatible_ids": sorted(incompatible),
        "candidate_count": len(candidates),
        "with_package_evidence_count": len(with_package),
        "without_package_evidence_count": len(without_package),
        "without_github_stars_count": sum(row["github_stars"] is None for row in candidates),
        "shortfall_with_cataloged_packages": max(0, target - len(verified) - len(with_package)),
        "shortfall_even_if_all_candidates_pass": max(0, target - len(verified) - len(candidates)),
        "package_evidence_scope": "Solo OS. Architettura, ricetta e ciclo dei candidati da verificare.",
        "with_package_evidence": with_package,
        "without_package_evidence": without_package,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("platform", choices=TARGETS)
    parser.add_argument("--target", type=int, default=300, help="Obiettivo di cicli per piattaforma")
    parser.add_argument("--catalog", type=Path, default=ROOT / "crates/core/curated.json")
    parser.add_argument(
        "--verified", type=Path,
        default=ROOT / "apps/shiftanything-tauri-proof/verified-platforms.json",
    )
    args = parser.parse_args()
    if args.target < 1:
        parser.error("--target deve essere positivo")
    platform, arch = args.platform.split("-")
    try:
        result = plan(read_json(args.catalog), read_json(args.verified), platform, arch, args.target)
    except (OSError, ValueError) as error:
        parser.error(str(error))
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
