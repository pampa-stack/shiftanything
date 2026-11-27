# /// script
# requires-python = ">=3.11"
# dependencies = ["mcp==1.30.0"]
# ///
"""Client for the native test's own DMG fixture; the Rust test clicks the actual egui approvals."""
import asyncio
import json
import sys
from pathlib import Path
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

async def main():
    config = json.loads(Path(sys.argv[1]).read_text())["mcpServers"]["shiftanything"]
    params = StdioServerParameters(command=config["command"], args=config["args"])
    async with stdio_client(params) as (reader, writer):
        async with ClientSession(reader, writer) as session:
            await session.initialize()
            async def call(name, **args):
                answer = await session.call_tool(name, args)
                assert not answer.isError, answer
                return answer.structuredContent
            initial = await call("get_inventory")
            assert not next(app for app in initial["apps"] if app["id"] == "photocraft")["installed"]
            for operation in ["update", "update_all", "open"]:
                assert (await call("request_action", operation=operation, app_ids=["photocraft"]))["error"] == "invalid_native_state"
            rejected = await call("request_action", operation="install", app_ids=["photocraft"])
            assert "request_id" in rejected, rejected
            for _ in range(600):
                status = await call("get_request_status", request_id=rejected["request_id"])
                if status.get("state") == "rejected":
                    print("PASS real UI rejection before installation", flush=True)
                    break
                await asyncio.sleep(0.05)
            else:
                raise TimeoutError("rejection")
            for operation in ["install", "open", "update", "remove"]:
                # Native DMG work can block this test adapter; wait for a fresh observed state.
                for _ in range(600):
                    inventory = await call("get_inventory")
                    if not inventory.get("stale", True):
                        break
                    await asyncio.sleep(0.05)
                else:
                    raise TimeoutError("fresh native inventory")
                queued = await call("request_action", operation=operation, app_ids=["photocraft"])
                assert "request_id" in queued, queued
                assert not queued["completed"]
                for _ in range(600):
                    status = await call("get_request_status", request_id=queued["request_id"])
                    outcomes = status.get("outcomes", {})
                    outcome = outcomes.get(queued["request_id"] + ":photocraft", "")
                    assert not outcome.startswith("error:"), status
                    if outcome.startswith("completed:"):
                        print(json.dumps({"operation": operation, "request_id": queued["request_id"], "outcome": outcome}), flush=True)
                        break
                    assert status.get("state") != "rejected", status
                    await asyncio.sleep(0.05)
                else:
                    raise TimeoutError(operation)
            final = await call("get_inventory")
            assert not next(app for app in final["apps"] if app["id"] == "photocraft")["installed"]
            print("PASS real stdio → egui approval → own native DMG fixture install/open/update/remove", flush=True)

if __name__ == "__main__":
    asyncio.run(main())
