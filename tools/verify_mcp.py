# /// script
# requires-python = ">=3.11"
# dependencies = ["mcp==1.30.0"]
# ///
"""Exercise the real stdio transport, with repository-scoped roots and no user data."""
import asyncio
import sys
import json
import tempfile
import time
from pathlib import Path
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

async def main():
    root = Path(__file__).resolve().parents[1]
    expected_total = len(json.loads((root / "crates/core/curated.json").read_text()))
    params = StdioServerParameters(command=sys.executable, args=[str(root / "tools/mcp_catalogo.py"), "--automation-read-root", str(root), "--automation-write-root", str(root)])
    async with stdio_client(params) as (reader, writer):
        async with ClientSession(reader, writer) as session:
            await session.initialize()
            tools = (await session.list_tools()).tools
            assert {tool.name for tool in tools} == {"catalog_filters", "search_apps", "get_app", "comparison_profiles", "compare_apps"}
            assert all(tool.annotations.readOnlyHint for tool in tools)
            filters = (await session.call_tool("catalog_filters", {})).structuredContent
            assert filters["total"] == expected_total
            results = (await session.call_tool("search_apps", {"query": "Handy", "publisher": "CJ Pais", "level": "base"})).structuredContent
            assert any(app["id"] == "handy" for app in results["apps"])
            for app_id in ["prompticraft", "eyecraft", "oneshotanything", "unknown-app"]:
                result = (await session.call_tool("get_app", {"app_id": app_id})).structuredContent
                assert result["error"] == "unknown_app"
            for query in ["Granola", "Linear", "OCR"]:
                results = (await session.call_tool("search_apps", {"query": query})).structuredContent
                assert results["total"] > 0, query
            flathub = (await session.call_tool("search_apps", {"query": "LibreOffice"})).structuredContent
            office = next(app for app in flathub["apps"] if app["id"] == "libreoffice")
            assert office["popularity"]["kind"] == "flathub_installs" and office["popularity"]["page"].startswith("https://flathub.org/apps/")
            meetily = (await session.call_tool("get_app", {"app_id": "meetily"})).structuredContent["app"]
            assert meetily["agent"]["kind"] != "mcp"
            tenacity = (await session.call_tool("get_app", {"app_id": "tenacity"})).structuredContent["app"]
            assert tenacity["platform_support"]["windows"] and tenacity["distribution"][0]["download_size"] > 0
            profiles = (await session.call_tool("comparison_profiles", {"query": "PHÓTOSHÓP®"})).structuredContent
            assert [profile["id"] for profile in profiles["profiles"]] == ["photoshop"]
            all_profiles = (await session.call_tool("comparison_profiles", {})).structuredContent["profiles"]
            seed = json.loads((root / "crates/core/shifter.json").read_text())
            assert {p["id"] for p in all_profiles} == {p["id"] for p in seed["profiles"]}
            async def comparison(**args):
                return (await session.call_tool("compare_apps", args)).structuredContent
            result = await comparison(profile_id="photoshop", selected_features=["layers", "reversible", "psd", "genfill"], required_features=["genfill"])
            photo = next(row for row in result["results"] if row["app_id"] == "photocraft")
            assert photo["coverage"] == 75 and photo["missing_required"] == ["genfill"]
            assert all(evidence.get("source") for evidence in photo["evidence"].values())
            result = await comparison(profile_id="photoshop", selected_features=["layers", "reversible", "psd"])
            assert next(row for row in result["results"] if row["app_id"] == "photocraft")["coverage"] == 100
            result = await comparison(profile_id="photoshop")
            assert all(row["coverage"] is None for row in result["results"])
            assert (await comparison(profile_id="missing"))["error"] == "unknown_profile"
            assert (await comparison(profile_id="photoshop", selected_features=["missing"]))["error"] == "invalid_selection"
            assert (await comparison(profile_id="photoshop", required_features=["layers"]))["error"] == "invalid_selection"
            manual = await comparison(software_name="Photoshop", app_ids=["photocraft"], selected_features=["custom feature"])
            assert manual["manual"] and manual["results"][0]["coverage"] is None
            manual = await comparison(software_name="An unknown product", app_ids=["photocraft"], selected_features=["photoshop:layers", "custom:fixture"])
            assert manual["results"][0]["coverage"] == 50 and manual["results"][0]["evidence"]["photoshop:layers"]["source"]

            for profile in all_profiles:
                selected = [feature["id"] for feature in profile["features"]]
                result = await comparison(profile_id=profile["id"], selected_features=selected, required_features=selected)
                assert result["results"], profile["id"]
                assert all(sum(row[k] for k in ["supported", "partial", "unsupported", "unverified"]) == len(selected) for row in result["results"])
            print(f"PASS sh*fter stdio: {len(all_profiles)} profiles; explicit/manual selection and evidence reuse")
            print(f"PASS stdio MCP: {filters['total']} apps, combined filters, unknown/deferred IDs rejected, exactly 5 read-only tools")


    with tempfile.TemporaryDirectory(prefix=".mcp-proof-", dir=root) as folder:
        bridge = Path(folder)
        app = dict(id="handy", detected=True, installed=False, compatible=True, installable=True, update_available=False, job=None)
        state = dict(updated_at=int(time.time()), apps=[app], results={})
        def publish():
            state["updated_at"] = int(time.time())
            (bridge / "snapshot.json").write_text(json.dumps(state))
        publish()
        params.args += ["--bridge-root", folder]
        async with stdio_client(params) as (reader, writer):
            async with ClientSession(reader, writer) as session:
                await session.initialize()
                async def call(name, **args):
                    result = await session.call_tool(name, args)
                    assert not result.isError, result
                    return result.structuredContent
                for op in ["update", "update_all", "open", "remove", "cancel"]:
                    assert (await call("request_action", operation=op, app_ids=["handy"]))["error"] == "invalid_native_state"
                    assert not (bridge / "request.json").exists()
                assert (await call("request_action", operation="install", app_ids=["unknown"]))["error"] == "unknown_native_recipe"
                assert (await call("get_request_status", request_id="unknown-task"))["error"] == "unknown_request"
                app["compatible"] = False
                publish()
                assert (await call("request_action", operation="install", app_ids=["handy"]))["error"] == "invalid_native_state"
                app["compatible"] = True
                app["installed"] = True
                publish()
                assert (await call("request_action", operation="install", app_ids=["handy"]))["error"] == "invalid_native_state"
                app["update_available"] = True
                publish()
                queued = await call("request_action", operation="update_all", app_ids=["handy"])
                request = json.loads((bridge / "request.json").read_text())
                assert request["operation"] == "update_all" and request["id"] == queued["request_id"]
                assert not queued["completed"]
                assert (await call("request_action", operation="update", app_ids=["handy"]))["error"] == "approval_queue_busy"
                (bridge / "request.json").unlink()
                state["results"][request["id"]] = "rejected"
                publish()
                assert (await call("get_request_status", request_id=request["id"]))["state"] == "rejected"
                app["job"] = "Downloading"
                publish()
                cancel = await call("request_action", operation="cancel", app_ids=["handy"])
                assert "request_id" in cancel
                (bridge / "request.json").unlink()
                # These are explicit IPC fixtures, not claims of native execution.
                state["results"][cancel["request_id"]] = "approved; native operation requested (not completed)"
                state["results"][cancel["request_id"] + ":handy"] = "error: cancelled"
                publish()
                outcome = await call("get_request_status", request_id=cancel["request_id"])
                assert outcome["outcomes"][cancel["request_id"] + ":handy"] == "error: cancelled"
                app["job"] = None
                publish()
                (bridge / "request.json").symlink_to(bridge / "snapshot.json")
                assert (await call("request_action", operation="open", app_ids=["handy"]))["error"] == "approval_queue_busy"
                (bridge / "request.json").unlink()
                assert not list(bridge.glob(".request-*"))
                for _ in range(20):
                    queued = await call("request_action", operation="open", app_ids=["handy"])
                    assert json.loads((bridge / "request.json").read_text())["id"] == queued["request_id"]
                    (bridge / "request.json").unlink()
                print("PASS stdio action bridge with explicit temporary IPC fixtures: state rejection, bounded queue, cancellation, unknown IDs/tasks, outcomes, symlink rejection and complete publications")

    if len(sys.argv) > 1:
        config = json.loads(Path(sys.argv[1]).read_text())["mcpServers"]["shiftanything"]
        packaged = StdioServerParameters(command=config["command"], args=config["args"])
        async with stdio_client(packaged) as (reader, writer):
            async with ClientSession(reader, writer) as session:
                await session.initialize()
                result = (await session.call_tool("catalog_filters", {})).structuredContent
                assert result["total"] == expected_total
                inventory = (await session.call_tool("get_inventory", {})).structuredContent
                assert inventory["error"] == "launcher_unavailable"
                bridge = Path(config["args"][-1])
                assert abs(time.time() - json.loads((bridge / "client.json").read_text())["last_seen"]) < 60
                print(f"PASS actual native-packaged server/config: scoped startup, {expected_total} apps, real tool heartbeat and honest missing launcher")

if __name__ == "__main__":
    asyncio.run(main())
