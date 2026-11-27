# /// script
# requires-python = ">=3.11"
# dependencies = ["mcp==1.30.0"]
# ///
"""Exercise real stdio against a running isolated Tauri proof. Never approves actions."""
import argparse
import asyncio
import json
import os
import shutil
import sys
from pathlib import Path
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client


async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("profile", type=Path)
    parser.add_argument("--app", default="handy", help="Native catalog app ID used for this proof")
    parser.add_argument("--operation", choices=["install", "update", "remove"])
    args = parser.parse_args()
    profile = args.profile.resolve(strict=True)
    snapshot = profile / "native-state.json"
    assert snapshot.is_file() and not snapshot.is_symlink(), "Start the isolated Tauri proof first"
    state = json.loads(snapshot.read_text())
    assert state.get("isolated_profile") is True, "Use an isolated Tauri profile"
    bridge_paths = {
        "MacOs": "home/Library/Application Support/ShiftAnything/automation",
        "Linux": "home/data/shiftanything/automation",
    }
    relative = bridge_paths.get(state["platform"]["os"])
    assert relative, "This proof supports macOS and Linux"
    bridge = profile / relative
    assert bridge.is_dir() and not bridge.is_symlink()
    assert bridge.resolve(strict=True) == bridge, "Use a profile without directory aliases"
    support = bridge / "mcp"
    support.mkdir(exist_ok=True)
    assert not support.is_symlink()
    repo = Path(__file__).resolve().parents[1]
    for source in [repo / "tools/mcp_catalogo.py", repo / "tools/shifter.py", repo / "crates/core/curated.json", repo / "crates/core/shifter.json"]:
        destination = support / source.name
        assert not destination.is_symlink()
        shutil.copyfile(source, destination)
    env = os.environ.copy()
    env.update(HOME=str(profile / "home"), CFFIXED_USER_HOME=str(profile / "home"), PYTHONDONTWRITEBYTECODE="1")
    env.pop("ARTCRAFT_LAUNCHER_GITHUB_TOKEN", None)
    server = StdioServerParameters(command=sys.executable, args=[str(support / "mcp_catalogo.py"), "--automation-read-root", str(bridge), "--automation-write-root", str(bridge), "--bridge-root", str(bridge)], env=env)
    async with stdio_client(server) as (reader, writer):
        async with ClientSession(reader, writer) as session:
            await session.initialize()

            async def call(name, **arguments):
                answer = await session.call_tool(name, arguments)
                assert not answer.isError, answer
                return answer.structuredContent

            state = await call("get_inventory")
            assert not state.get("stale", True), state
            app = next(app for app in state["apps"] if app["id"] == args.app)
            assert app["detected"] and not app["job"], app
            invalid = ["install", "cancel"] if app["installed"] else ["open", "update", "update_all", "remove", "cancel"]
            if app["installed"] and not app["update_available"]:
                invalid += ["update", "update_all"]
            for operation in invalid:
                result = await call("request_action", operation=operation, app_ids=[args.app])
                assert result.get("error") == "invalid_native_state", result
                print(f"PASS rejected {operation} with installed={app['installed']}", flush=True)
            assert (await call("request_action", operation="install", app_ids=["unknown-app"]))["error"] == "unknown_native_recipe"
            assert (await call("get_request_status", request_id="unknown-task"))["error"] == "unknown_request"
            print("PASS unknown app/task rejected", flush=True)
            if not args.operation:
                return
            queued = await call("request_action", operation=args.operation, app_ids=[args.app])
            assert "request_id" in queued and queued["completed"] is False, queued
            print(json.dumps(queued), flush=True)
            deadline = asyncio.get_running_loop().time() + 180
            while asyncio.get_running_loop().time() < deadline:
                result = await call("get_request_status", request_id=queued["request_id"])
                outcomes = result.get("outcomes", {})
                outcome = outcomes.get(f"{queued['request_id']}:{args.app}", "")
                if result.get("state") == "rejected":
                    print("PASS explicit desktop rejection; no native action", flush=True)
                    return
                if outcome.startswith("completed:"):
                    print(json.dumps(result), flush=True)
                    return
                assert not outcome.startswith(("failed:", "rejected:")), result
                await asyncio.sleep(.2)
            raise TimeoutError("No desktop decision/native outcome within 180 seconds")


if __name__ == "__main__":
    asyncio.run(main())
