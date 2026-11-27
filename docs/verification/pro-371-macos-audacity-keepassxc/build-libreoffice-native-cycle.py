"""Link the disposable native proof against cargo's actual build artifacts."""
import json
from pathlib import Path
import subprocess

proof = Path(__file__).resolve().parent
artifacts = {}
wanted = {"artcraft_launcher_core", "artcraft_launcher_native"}
for line in (proof / "libreoffice-cargo-build.jsonl").read_text().splitlines():
    data = json.loads(line)
    if data["reason"] != "compiler-artifact" or data["target"]["name"] not in wanted:
        continue
    for artifact in data["filenames"]:
        linked = Path(artifact)
        if linked.suffix in {".rlib", ".rmeta"} and linked.with_suffix(".rlib").is_file():
            artifacts[data["target"]["name"]] = linked.with_suffix(".rlib")
assert artifacts.keys() == wanted, artifacts
deps = artifacts["artcraft_launcher_native"].parent
command = [
    str(Path.home() / ".cargo/bin/rustc"),
    "--edition",
    "2024",
    "-L",
    f"dependency={deps}",
    str(proof / "libreoffice-native-cycle.rs"),
    "-o",
    str(proof / "libreoffice-native-cycle"),
]
for name, path in artifacts.items():
    command += ["--extern", f"{name}={path}"]
subprocess.run(command, check=True)
