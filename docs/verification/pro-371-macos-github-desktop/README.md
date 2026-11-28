# GitHub Desktop on Apple Silicon

NativeBackend installed release 3.7.1, removed it, updated the signed 3.7.0
migration fixture to 3.7.1, then removed 3.7.1 to the private Trash. The
synthetic user-data marker survived. [Native-cycle output](native-cycle.log).

- MacBook Air with macOS 26.6.2 and Gatekeeper enabled. The launcher runs over SSH with its own temporary home, Applications directory, download cache and Trash.
- [Official releases](https://github.com/desktop/desktop/releases) publish `GitHub.Desktop-arm64.zip` under the exact tags `release-3.7.0` and `release-3.7.1`. The two real downloads match GitHub’s published SHA-256 and are 538,188,819 and 556,131,016 bytes respectively.
- Both archives contain native arm64 `GitHub Desktop.app`, with identifier `com.github.GitHubClient`, Developer ID team `VEKTX9H2N7`, strict recursive signatures and Gatekeeper approval as Notarized Developer ID.
- The larger archive expands to 1,211,690,181 bytes. The recipe uses this measured extraction limit. The 768 MiB cap applies to compressed downloads and other apps. Framework links resolve within their framework.
- Opening the GUI was outside this isolated test. No graphical session was used.
- The current 3.7.1 ZIP is the only launcher install option. The signed 3.7.0 bundle is used solely to test migration after verifying its identifier, signature, notarization, architecture and version. The harness seeds a managed receipt inside the temporary profile so its isolated inventory can see the bundle.
