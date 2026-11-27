"""Prove inherited release entrypoints stop before filesystem or signing work."""
from pathlib import Path
import subprocess
import tempfile

root = Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory(prefix="shift-release-guard-") as folder:
    env = {"PATH": "/usr/bin:/bin", "HOME": folder,
           "DIST": str(Path(folder) / "dist"), "CARGO_TARGET_DIR": str(Path(folder) / "target")}
    for relative in ["macos/package.sh", "macos/import-cert.sh", "linux/package.sh", "freebsd/package.sh", "web/package.sh"]:
        result = subprocess.run(["/bin/bash", str(root / "packaging" / relative), "--skip-build"],
                                env=env, capture_output=True, text=True, timeout=10)
        assert result.returncode != 0 and "Upstream release packaging is disabled" in result.stderr, relative
        assert not list(Path(folder).iterdir()), f"unexpected files after {relative}"
        print("PASS release entrypoint blocked without filesystem/signing work:", relative)
workflow = (root / ".github/workflows/release.yml").read_text()
assert "secrets:" not in workflow and "uses:" not in workflow and "contents: write" not in workflow
print("PASS no reusable release call, signing-secret forwarding or publishing permissions")
print("NOT TESTED: Windows PowerShell execution; its package/sign entrypoints have an explicit throw")
