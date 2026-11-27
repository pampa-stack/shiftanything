# /// script
# requires-python = ">=3.11"
# dependencies = ["mcp==1.30.0"]
# ///
"""Opt-in real official Handy cycle. Driven by verify_official_mcp Rust example only."""
import asyncio
import json
import sys
from pathlib import Path
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

async def main():
    config = json.loads(Path(sys.argv[1]).read_text())["mcpServers"]["shiftanything"]
    root = Path(sys.argv[2])
    async with stdio_client(StdioServerParameters(command=config["command"], args=config["args"])) as (reader, writer):
        async with ClientSession(reader, writer) as session:
            await session.initialize()
            async def call(name, **args):
                answer = await session.call_tool(name, args)
                assert not answer.isError, answer
                return answer.structuredContent
            async def ready(installed):
                for _ in range(1200):
                    inventory = await call("get_inventory")
                    app = next(app for app in inventory.get("apps", []) if app["id"] == "handy")
                    if not inventory.get("stale", True) and app["detected"] and app["installed"] == installed and not app["job"]:
                        return app
                    await asyncio.sleep(.1)
                raise TimeoutError("native inventory")
            async def request(operation, rejected=False):
                queued = await call("request_action", operation=operation, app_ids=["handy"])
                assert "request_id" in queued, queued
                assert not queued["completed"]
                for _ in range(1800):
                    status = await call("get_request_status", request_id=queued["request_id"])
                    if rejected and status.get("state") == "rejected":
                        print("PASS actual egui rejection before native update", flush=True)
                        return
                    outcome = status.get("outcomes", {}).get(queued["request_id"] + ":handy", "")
                    assert not outcome.startswith("error:"), status
                    if outcome.startswith("completed:"):
                        print(json.dumps(dict(operation=operation, outcome=outcome)), flush=True)
                        return
                    await asyncio.sleep(.1)
                raise TimeoutError(operation)
            await ready(True)
            assert (await call("request_action", operation="install", app_ids=["handy"]))["error"] == "invalid_native_state"
            await request("update", rejected=True)
            launches = 0
            operations = ["update", "remove", "install", "remove"] if "--no-open" in sys.argv else ["update", "open", "remove", "install", "open", "remove"]
            for operation in operations:
                await ready(operation != "install")
                await request(operation)
                if operation == "update":
                    await ready(True)
                    for op in ["update", "update_all"]:
                        assert (await call("request_action", operation=op, app_ids=["handy"]))["error"] == "invalid_native_state"
                elif operation == "open":
                    launches += 1
                    for _ in range(300):
                        if (root / f"launch-{launches}").exists():
                            break
                        await asyncio.sleep(.1)
                    else:
                        raise TimeoutError("actual isolated process observation")
                elif operation == "remove":
                    await ready(False)
                    for op in ["open", "update", "update_all"]:
                        assert (await call("request_action", operation=op, app_ids=["handy"]))["error"] == "invalid_native_state"
            print("PASS real stdio official package operations with actual NativeBackend and egui approvals; open=" + str("--no-open" not in sys.argv), flush=True)

if __name__ == "__main__":
    asyncio.run(main())
