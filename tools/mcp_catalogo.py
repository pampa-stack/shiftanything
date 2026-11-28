# /// script
# requires-python = ">=3.11"
# dependencies = ["mcp==1.30.0"]
# ///
"""Serve the ShiftAnything catalog to a local MCP client over stdio."""

import json
import unicodedata
from collections import Counter
from pathlib import Path
from typing import Annotated, Any, Literal

from mcp.server.fastmcp import FastMCP
from mcp.types import ToolAnnotations
from pydantic import Field

CATALOG_PATH = Path(__file__).resolve().with_name("curated.json")
if not CATALOG_PATH.is_file():
    CATALOG_PATH = Path(__file__).resolve().parents[1] / "crates" / "core" / "curated.json"
READ_ONLY = ToolAnnotations(readOnlyHint=True, idempotentHint=True, openWorldHint=False)
server = FastMCP(
    "Sh*ftAnything",
    instructions=(
        "Discover free and open-source software. Call catalog_filters to see available categories, "
        "search_apps to find apps or alternatives, and get_app for verified source links and package details. "
        "Use the user's language. Treat GitHub stars as a popularity signal. Only report a platform, "
        "package or integration as documented when the app record lists it. This catalog server reads local data."
    ),
)


from shifter import compare as compare_evidence, manual_compare, normalize as comparison_normalize, profile_evidence

SHIFTER_PATH = Path(__file__).resolve().with_name("shifter.json")
if not SHIFTER_PATH.is_file():
    SHIFTER_PATH = Path(__file__).resolve().parents[1] / "crates" / "core" / "shifter.json"


def comparison_data():
    MARK_CLIENT()
    return json.loads(SHIFTER_PATH.read_text(encoding="utf-8"))


def comparison_results(results, evidence, selected, *, shortlist=False):
    rows = []
    for row in results:
        if not shortlist and (row["coverage"] or 0) <= 0:
            continue
        known = evidence.get(row["app_id"], {})
        if shortlist:
            sources = {feature: known.get(feature, {"state": "unverified"}) for feature in selected}
        else:
            sources = {feature: known[feature] for feature in selected if feature in known}
        rows.append(dict(row, evidence=sources))
    return rows


@server.tool(annotations=READ_ONLY, structured_output=True)
def comparison_profiles(query: Annotated[str, Field(max_length=240)] = "") -> dict[str, Any]:
    """List sh*fter profiles and feature IDs. A search result is not a selected profile."""
    data = comparison_data()
    needle = comparison_normalize(query)
    return {"checked_on": data["checked_on"], "profiles": [
        {key: profile[key] for key in ["id", "name", "aliases", "features"]}
        for profile in data["profiles"]
        if not needle or any(needle in comparison_normalize(name) for name in [profile["name"], *profile["aliases"]])
    ]}


@server.tool(annotations=READ_ONLY, structured_output=True)
def compare_apps(
    profile_id: Annotated[str, Field(max_length=80)] = "",
    selected_features: Annotated[list[str], Field(max_length=32)] = [],
    required_features: Annotated[list[str], Field(max_length=32)] = [],
    software_name: Annotated[str, Field(max_length=240)] = "",
    app_ids: Annotated[list[str], Field(max_length=30)] = [],
) -> dict[str, Any]:
    """Compare documented features using an explicitly selected profile, or a manual name.

    Catalog-wide comparisons return only alternatives with positive documented coverage.
    To inspect specific apps including zero or unverified coverage, pass an app_ids
    shortlist with a manual software_name.
    Coverage is rounded half up: supported / ALL selected features. Partial, unsupported,
    and unverified are separate. No percentage for empty selection or all unverified.
    Omitted evidence entries are unverified. This tool never installs or changes evidence.
    """
    if any(len(feature) > 240 for feature in [*selected_features, *required_features]):
        return {"error": "invalid_feature"}
    if any(len(app_id) > 80 for app_id in app_ids) or len(set(app_ids)) != len(app_ids):
        return {"error": "unknown_or_duplicate_app"}
    apps = catalog()
    if app_ids and not set(app_ids) <= {app['id'] for app in apps}:
        return {"error": "unknown_or_duplicate_app"}
    data = comparison_data()
    selection = {"selected_features": selected_features, "required_features": required_features,
                 "default_evidence_state": "unverified"}
    if profile_id:
        profile = next((profile for profile in data["profiles"] if profile['id'] == profile_id), None)
        if profile is None:
            return {"error": "unknown_profile"}
        if app_ids:
            return {"error": "app_ids_are_for_manual_comparison"}
        try:
            results = compare_evidence(profile, apps, selected_features, required_features)
        except ValueError as error:
            return {"error": "invalid_selection", "message": str(error)}
        evidence = profile_evidence(profile)
        return {"profile_id": profile_id, "checked_on": data['checked_on'], "manual": False,
                **selection, "results": comparison_results(results, evidence, selected_features)}
    if not software_name.strip():
        return {"error": "select_profile_or_manual_name"}
    try:
        results, evidence = manual_compare(data, apps, app_ids, selected_features, required_features)
    except ValueError as error:
        return {"error": "invalid_selection", "message": str(error)}
    return {"manual": True, "software_name": software_name, "checked_on": data['checked_on'],
            **selection, "results": comparison_results(results, evidence, selected_features, shortlist=bool(app_ids))}



MARK_CLIENT = lambda: None


def catalog():
    MARK_CLIENT()
    return json.loads(CATALOG_PATH.read_text(encoding="utf-8"))


def normalize(value: str) -> str:
    return "".join(c for c in unicodedata.normalize("NFKD", value.casefold()) if not unicodedata.combining(c))


@server.tool(annotations=READ_ONLY, structured_output=True)
def catalog_filters() -> dict[str, Any]:
    """List available authors, categories and levels with their app counts."""
    apps = catalog()
    return {
        "total": len(apps),
        "authors": dict(sorted(Counter(a["author"] for a in apps).items(), key=lambda pair: pair[0].casefold())),
        "categories": dict(Counter(a["category"] for a in apps)),
        "levels": dict(Counter(a["level"] for a in apps)),
    }


@server.tool(annotations=READ_ONLY, structured_output=True)
def search_apps(
    query: Annotated[str, Field(max_length=240)] = "",
    publisher: str = "",
    category: str = "",
    level: Literal["base", "avanz", ""] = "",
    control: Literal["mcp", "cli", "api", "unknown", ""] = "",
    sort: Literal["featured", "name", "stars", "latest"] = "featured",
    language: Literal["it", "en"] = "it",
    limit: Annotated[int, Field(ge=1, le=30)] = 15,
    offset: Annotated[int, Field(ge=0)] = 0,
) -> dict[str, Any]:
    """Find apps by name, category, publisher, alternative to a product, or agent control."""
    words = normalize(query).split()

    def matches(app):
        if publisher and normalize(publisher) != normalize(app["author"]):
            return False
        if category and category != app["category"]:
            return False
        if level and level != app["level"]:
            return False
        if control and control != app["agent"]["kind"]:
            return False
        text = normalize(" ".join((app["name"], app["author"], app["category"], *app["alternatives"], *app["description"].values())))
        return all(word in text for word in words)

    found = [a for a in catalog() if matches(a)]
    if sort == "name":
        found.sort(key=lambda a: normalize(a["name"]))
    elif sort == "stars":
        found.sort(key=lambda a: a.get("stars", {}).get("count", -1), reverse=True)
    elif sort == "latest":
        found.sort(key=lambda a: a.get("release", {}).get("date") or "", reverse=True)

    keys = ("id", "name", "author", "category", "level", "license", "repo", "agent", "stars", "popularity", "delivery", "platform_support", "distribution")
    page = [{**{key: app[key] for key in keys if key in app}, "description": app["description"][language]}
            for app in found[offset:offset + limit]]
    next_offset = offset + limit if offset + limit < len(found) else None
    return {"total": len(found), "next_offset": next_offset, "apps": page}


@server.tool(annotations=READ_ONLY, structured_output=True)
def get_app(app_id: str) -> dict[str, Any]:
    """Read the full bilingual record for an app ID returned by search_apps."""
    app = next((a for a in catalog() if a["id"] == app_id), None)
    return {"app": app} if app else {"error": "unknown_app", "app_id": app_id}


def configure_bridge(root: Path):
    """Only these fixed IPC files are accessible; requests never carry shell text or paths."""
    import os
    import time
    import uuid
    import stat
    import tempfile

    def mark_client():
        fd, staging = tempfile.mkstemp(prefix=".client-", dir=root)
        try:
            with os.fdopen(fd, "w", encoding="utf-8") as stream:
                json.dump({"last_seen": int(time.time())}, stream)
            os.replace(staging, root / "client.json")
        finally:
            if os.path.exists(staging):
                os.unlink(staging)

    global MARK_CLIENT
    MARK_CLIENT = mark_client

    def read_file(name):
        path = root / name
        fd = os.open(path, os.O_RDONLY | getattr(os, "O_NOFOLLOW", 0))
        with os.fdopen(fd, "rb") as stream:
            metadata = os.fstat(stream.fileno())
            if not stat.S_ISREG(metadata.st_mode) or metadata.st_nlink != 1:
                raise ValueError("unexpected bridge file type or link")
            data = stream.read(1_048_577)
        if len(data) > 1_048_576:
            raise ValueError("bridge response too large")
        return json.loads(data)

    def snapshot():
        mark_client()
        try:
            result = read_file("snapshot.json")
        except (OSError, ValueError):
            return {"error": "launcher_unavailable"}
        result["stale"] = abs(time.time() - result.get("updated_at", 0)) > 10
        return result

    @server.tool(annotations=READ_ONLY, structured_output=True)
    def get_inventory() -> dict[str, Any]:
        """Read native versions, jobs and observed outcomes. A stale snapshot is not current state."""
        return snapshot()

    @server.tool(annotations=READ_ONLY, structured_output=True)
    def get_request_status(request_id: Annotated[str, Field(max_length=64)]) -> dict[str, Any]:
        """Approval is not completion. Native outcomes are reported separately for each app."""
        state = snapshot()
        if "error" in state:
            return state
        results = state.get("results", {})
        if request_id in results:
            return {"state": results[request_id], "stale": state["stale"], "outcomes": {key: value for key, value in results.items() if key.startswith(request_id + ":")}}
        try:
            pending = read_file("request.json")
            if pending.get("id") == request_id:
                return {"state": "queued; awaiting launcher", "stale": state["stale"]}
        except (OSError, ValueError):
            pass
        return {"error": "unknown_request"}

    @server.tool(annotations=ToolAnnotations(readOnlyHint=False, destructiveHint=True, idempotentHint=False, openWorldHint=False), structured_output=True)
    def request_action(operation: Literal["install", "update", "open", "remove", "update_all", "cancel"], app_ids: Annotated[list[str], Field(min_length=1, max_length=32)]) -> dict[str, Any]:
        """Queue a curated native action for explicit desktop approval; never executes it here."""
        if len(set(app_ids)) != len(app_ids) or (operation != "update_all" and len(app_ids) != 1):
            return {"error": "invalid_app_count"}
        state = snapshot()
        if "error" in state or state.get("stale"):
            return {"error": "launcher_unavailable_or_stale"}
        native = {app["id"]: app for app in state.get("apps", [])}
        curated = {app["id"] for app in catalog()}
        for app_id in app_ids:
            app = native.get(app_id)
            if app_id not in curated or not app:
                return {"error": "unknown_native_recipe", "app_id": app_id}
            busy = app.get("job") is not None
            allowed = app.get("detected") and (not busy or operation == "cancel")
            if operation == "install":
                allowed = allowed and not app.get("installed") and app.get("compatible") and app.get("installable")
            elif operation in ("update", "update_all"):
                allowed = allowed and app.get("installed") and app.get("compatible") and app.get("installable") and app.get("update_available")
            elif operation in ("open", "remove"):
                allowed = allowed and app.get("installed") and app.get("can_" + operation, True)
            elif operation == "cancel":
                allowed = allowed and busy
            if not allowed:
                return {"error": "invalid_native_state", "app_id": app_id}
        request = {"id": str(uuid.uuid4()), "operation": operation, "apps": app_ids}
        # Publish only a closed, complete file. link() is atomic and never overwrites
        # a queued request (including an unexpected symlink).
        fd, staging = tempfile.mkstemp(prefix=".request-", dir=root)
        try:
            with os.fdopen(fd, "w", encoding="utf-8") as stream:
                json.dump(request, stream)
                stream.flush()
                os.fsync(stream.fileno())
            try:
                os.link(staging, root / "request.json", follow_symlinks=False)
            except FileExistsError:
                return {"error": "approval_queue_busy"}
        finally:
            os.unlink(staging)
        return {"request_id": request["id"], "state": "queued; explicit UI approval required", "completed": False}


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Sh*ftAnything scoped catalog MCP with optional desktop approval bridge")
    parser.add_argument("--automation-read-root", required=True, type=Path)
    parser.add_argument("--automation-write-root", required=True, type=Path)
    parser.add_argument("--bridge-root", type=Path)
    args = parser.parse_args()
    read_root = args.automation_read_root.resolve(strict=True)
    write_root = args.automation_write_root.resolve(strict=True)
    if not read_root.is_dir() or not write_root.is_dir():
        parser.error("automation roots must be existing directories")
    if not CATALOG_PATH.resolve(strict=True).is_relative_to(read_root):
        parser.error("catalog is outside automation-read-root")
    if not SHIFTER_PATH.resolve(strict=True).is_relative_to(read_root):
        parser.error("comparison data is outside automation-read-root")
    if args.bridge_root:
        bridge = args.bridge_root.resolve(strict=True)
        if args.bridge_root.is_symlink() or not bridge.is_dir() or not bridge.is_relative_to(read_root) or not bridge.is_relative_to(write_root):
            parser.error("bridge must be a directory within both automation roots")
        configure_bridge(bridge)

    server.run(transport="stdio")
