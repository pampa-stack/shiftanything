"""Controlla i rilasci stabili e raccoglie proposte SH*FTER dai changelog ufficiali."""

import argparse
import json
import re
import subprocess
from collections import defaultdict
from datetime import UTC, date, datetime
from html.parser import HTMLParser
from pathlib import Path
from tempfile import TemporaryDirectory
from urllib.parse import unquote, urlsplit
from urllib.request import HTTPRedirectHandler, build_opener

ROOT = Path(__file__).resolve().parents[1]
REPO = re.compile(r"https://github\.com/([\w.-]+/[\w.-]+)/?")
FLATPAK = re.compile(r"https://flathub\.org/apps/([\w.-]+)")
VERSION = re.compile(r"(?:[vV]|release-)?(\d+(?:\.\d+)*)(?:\+[\w.-]+)?")
PREVIEW = re.compile(r"\b(?:alpha|beta|nightly|preview|pre[- ]?release|rc)\d*\b", re.I)
PREVIEW_TAG = re.compile(r"^((?:v|release-)?\d+(?:\.\d+)*)-(?:alpha|beta|nightly|preview|pre-release|prerelease|rc)\d*(?:[.-]\d+)*$", re.I)
FEATURE = re.compile(
    r"^(?:feat(?:\([^)]*\))?!?:|add(?:s|ed)?\b|introduc(?:e|es|ed)\b|"
    r"supports?\b|new\b|remove(?:s|d)?\b|deprecat(?:e|es|ed)\b)", re.I
)
MAX_FLATHUB_BYTES = 4 * 1024 * 1024


class SameOriginRedirect(HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        dst = urlsplit(newurl)
        if dst.scheme != "https" or dst.netloc != urlsplit(req.full_url).netloc:
            raise ValueError("Redirect verso una fonte diversa")
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def flathub(path):
    with build_opener(SameOriginRedirect()).open(f"https://flathub.org/api/v2/{path}", timeout=20) as res:
        payload = res.read(MAX_FLATHUB_BYTES + 1)
        if len(payload) > MAX_FLATHUB_BYTES:
            raise ValueError(f"Risposta Flathub troppo grande: {path}")
        return json.loads(payload)


def github(repo):
    result = subprocess.run(
        ["gh", "api", "--hostname", "github.com", "-X", "GET", f"repos/{repo}/releases/latest"],
        check=False, capture_output=True, text=True, timeout=30,
    )
    if result.returncode:
        if "(HTTP 404)" in result.stderr:
            return None
        raise ValueError(f"GitHub API non disponibile per {repo}: {result.stderr.strip()}")
    return json.loads(result.stdout)


def package_platform(name):
    lower = name.lower()
    if lower.endswith(".dmg"):
        return "macos"
    if lower.endswith((".msi", "-setup.exe", "_setup.exe", "installer.exe")):
        return "windows"
    if lower.endswith((".appimage", ".deb", ".rpm", ".flatpak", ".pkg.tar.zst")) or re.search(
        r"-artifact-appimage-(?:x86_64|aarch64)\.zip$", lower
    ):
        return "linux"
    return None


def version_key(value):
    match = VERSION.fullmatch(value) if isinstance(value, str) else None
    if not match:
        return None
    components = tuple(map(int, match[1].split(".")))
    while len(components) > 1 and components[-1] == 0:
        components = components[:-1]
    return components


def github_release(repo, data):
    if not data or data.get("draft") or data.get("prerelease"):
        return None
    version = data["tag_name"]
    if version_key(version) is None or PREVIEW.search(data.get("name") or ""):
        return None

    def release_path(kind, suffix):
        return f"/{repo}/releases/{kind}/{suffix}"

    def official(url, path):
        parsed = urlsplit(url)
        prefix = f"/{repo}"
        return (parsed.scheme == "https" and parsed.netloc == "github.com"
                and not parsed.query and not parsed.fragment
                and parsed.path[:len(prefix)].lower() == prefix.lower()
                and unquote(parsed.path[len(prefix):]) == path[len(prefix):])

    if not official(data["html_url"], release_path("tag", version)):
        raise ValueError("Il rilascio non appartiene al repository richiesto")
    packages = defaultdict(list)
    for asset in data["assets"]:
        name, size = asset["name"], asset["size"]
        platform = package_platform(name)
        expected = release_path("download", f"{version}/{name}")
        if platform and type(size) is int and size > 0 and official(asset["browser_download_url"], expected):
            packages[platform].append(size)
    notes = list(markdown_lines(data.get("body") or "", repo))
    return {
        "version": version, "date": date.fromisoformat(data["published_at"][:10]).isoformat(),
        "source": data["html_url"],
        "packages": [
            {"platform": platform, "size_min": min(sizes), "size_max": max(sizes)}
            for platform, sizes in sorted(packages.items())
        ],
    }, notes


def markdown_lines(body, repo):
    fence = None
    for raw in body.splitlines():
        line = raw.strip()
        marker = re.match(r"^(`{3,}|~{3,})", line)
        if marker:
            if fence is None:
                fence = marker[1]
            elif marker[1].startswith(fence):
                fence = None
            continue
        item = re.match(r"^(?:[-*+]|\d+[.)])\s+(.+)$", line)
        if not fence and item:
            clean = re.sub(rf" by @[\w-]+ in https://github\.com/{re.escape(repo)}/pull/\d+$", "", item[1],
                           flags=re.I)
            if clean:
                yield clean


class ChangelogItems(HTMLParser):
    def __init__(self):
        super().__init__()
        self.lines = []
        self.current = None
        self.container = None

    def handle_starttag(self, tag, attrs):
        if tag in {"li", "p"} and self.current is None:
            self.current = []
            self.container = tag

    def handle_data(self, data):
        if self.current is not None:
            self.current.append(data)

    def handle_endtag(self, tag):
        if tag == self.container:
            self.lines.append(" ".join("".join(self.current).split()))
            self.current = None
            self.container = None


def flatpak_release(app_id):
    stream = flathub(f"appstream/{app_id}")
    if stream.get("id") != app_id:
        raise ValueError(f"Identità Flathub errata per {app_id}")
    releases = [item for item in stream.get("releases", [])
                if item.get("type") == "stable" and version_key(item.get("version")) is not None]
    if not releases:
        return None

    def released_on(item):
        if item.get("date"):
            return date.fromisoformat(item["date"]).isoformat()
        return datetime.fromtimestamp(int(item["timestamp"]), UTC).date().isoformat()

    latest = max(releases, key=lambda item: (released_on(item), version_key(item["version"])))
    notes = ChangelogItems()
    notes.feed(latest.get("description") or "")
    notes.close()
    return {
        "version": latest["version"], "date": released_on(latest),
        "source": f"https://flathub.org/apps/{app_id}", "packages": [],
    }, notes.lines


def update_flatpak_size(app, app_id, as_of):
    channel = next((entry for entry in (app.get("distribution") or [])
                    if entry["manager"] == "flatpak" and entry["id"] == app_id), None)
    if channel is None:
        return
    path = f"summary/{app_id}"
    size = flathub(path).get("download_size")
    source = f"https://flathub.org/api/v2/{path}"
    if type(size) is int and size > 0 and (
        channel.get("download_size") != size
        or channel.get("download_size_source") != source
        or not channel.get("download_size_checked")
    ):
        channel.update(download_size=size, download_size_source=source, download_size_checked=as_of)


def release_for(app):
    source = (app.get("release") or {}).get("source") or ""
    if flatpak := FLATPAK.fullmatch(source):
        return flatpak_release(flatpak[1])
    repo = REPO.fullmatch(app.get("repo") or "")
    if repo and (not source or source.lower().startswith(f"https://github.com/{repo[1]}/releases/".lower())):
        return github_release(repo[1], github(repo[1]))
    return None


def notes_baseline(document):
    if not isinstance(document, dict) or document.get("schema") != 1 or not isinstance(document.get("apps"), dict):
        raise ValueError("Baseline dei changelog non valida")
    entries = document["apps"]
    for entry in entries.values():
        if (
            not isinstance(entry, dict)
            or version_key(entry.get("version")) is None
            or not isinstance(entry.get("source"), str)
            or not isinstance(entry.get("lines"), list)
            or not all(isinstance(line, str) for line in entry["lines"])
        ):
            raise ValueError("Riga della baseline dei changelog non valida")
    return entries


def refresh(catalog, shifter, baseline, selected, as_of):
    """Update metadata in memory; the comparison data is only read for profile references."""
    notes = baseline.copy()
    profiles = defaultdict(list)
    for profile in shifter["profiles"]:
        for app in profile["candidates"]:
            profiles[app["app_id"]].append(profile["id"])
    report = {"checked": [], "changes": [], "unchanged": [], "skipped": []}
    proposals = []
    for app in catalog:
        app_id = app["id"]
        if selected and app_id not in selected:
            continue
        result = release_for(app)
        if result is None:
            report["skipped"].append({"id": app_id, "reason": "no_stable_supported_source"})
            continue
        before = app.get("release") or {}
        release, lines = result
        new_key = version_key(release["version"])
        old_version = before.get("version")
        old_key = version_key(old_version)
        previous_preview = PREVIEW_TAG.fullmatch(old_version) if old_key is None and isinstance(old_version, str) else None
        if previous_preview:
            old_key = version_key(previous_preview[1])
        if old_version and old_key is None:
            report["skipped"].append({"id": app_id, "reason": "incomparable_current_version"})
            continue
        same_version = old_key == new_key and previous_preview is None
        if old_version and not same_version and (new_key < old_key or release["date"] < (before.get("date") or "")):
            report["skipped"].append({"id": app_id, "reason": "rollback", "current": old_version,
                                      "observed": release["version"], "source": release["source"]})
            continue

        distribution_before = [channel.copy() for channel in (app.get("distribution") or [])]
        flatpak = FLATPAK.fullmatch(release["source"])
        if flatpak:
            update_flatpak_size(app, flatpak[1], as_of)
        if same_version:
            release["date"] = before.get("date") or release["date"]
            if not release["packages"]:
                release["packages"] = before.get("packages", [])
        report["checked"].append(app_id)
        distribution_changed = distribution_before != (app.get("distribution") or [])
        if before != release or distribution_changed:
            report["changes"].append({"id": app_id, "kind": "metadata" if same_version else "release",
                                      "before": before or None, "release": release,
                                      "distribution_changed": distribution_changed})
        else:
            report["unchanged"].append(app_id)
        app["release"] = release

        source = f"https://flathub.org/api/v2/appstream/{flatpak[1]}" if flatpak else release["source"]
        lines = list(dict.fromkeys(line for line in lines if FEATURE.match(line)))
        previous = baseline.get(app_id)
        if not same_version:
            changed_lines = lines
        elif previous and previous["source"] == source and version_key(previous["version"]) == new_key:
            old_lines = set(previous["lines"])
            changed_lines = [line for line in lines if line not in old_lines]
        else:
            changed_lines = []
        notes[app_id] = {"version": release["version"], "source": source, "lines": lines}
        if changed_lines:
            proposals.extend({"id": app_id, "version": release["version"], "source": source, "quote": line,
                              "profiles": profiles[app_id], "status": "needs_review"} for line in changed_lines)
    return report, proposals, {"schema": 1, "apps": notes}


def publish(output, files):
    if output.exists() or output.is_symlink():
        raise FileExistsError(f"Destinazione già presente: {output}")
    with TemporaryDirectory(dir=output.parent) as temporary:
        stage = Path(temporary) / "snapshot"
        stage.mkdir()
        for name, data in files.items():
            (stage / name).write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        output.mkdir(mode=0o700)
        # The report is written last and marks a complete snapshot.
        for name in files:
            (stage / name).rename(output / name)


def json_file(path):
    return json.loads(path.read_text(encoding="utf-8"))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--catalog", type=Path, default=ROOT / "crates/core/curated.json")
    parser.add_argument("--shifter", type=Path, default=ROOT / "crates/core/shifter.json")
    parser.add_argument("--baseline", type=Path, help="release-notes.json di un controllo precedente")
    parser.add_argument("--app", action="append", default=[], help="Limita il controllo a un ID ripetibile")
    parser.add_argument("--as-of", type=date.fromisoformat, default=datetime.now(UTC).date(),
                        help="Data della verifica delle dimensioni Flathub (AAAA-MM-GG, predefinito UTC)")
    parser.add_argument("--output", type=Path, help="Nuova directory per lo snapshot; senza, stampa soltanto")
    args = parser.parse_args(argv)
    try:
        catalog, shifter = json_file(args.catalog), json_file(args.shifter)
        if unknown := set(args.app) - {app["id"] for app in catalog}:
            parser.error(f"ID app inesistenti: {', '.join(sorted(unknown))}")
        baseline = notes_baseline(json_file(args.baseline)) if args.baseline else {}
        report, proposals, notes = refresh(catalog, shifter, baseline, set(args.app), args.as_of.isoformat())
        if args.output:
            publish(args.output, {"curated.json": catalog, "shifter-proposals.json": proposals,
                                  "release-notes.json": notes, "release-report.json": report})
    except (OSError, ValueError, KeyError, TypeError, subprocess.TimeoutExpired) as error:
        parser.exit(1, f"Snapshot interrotto: {error}\n")
    print(json.dumps({"catalog": report, "shifter_proposals": proposals}, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
