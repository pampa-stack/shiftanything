# OpenMTP on macOS Apple Silicon

November 28, 2026. MacBook Air, macOS 26.6.2, Gatekeeper enabled. `NativeBackend` installed OpenMTP 3.2.25, detected and updated it to 3.3.0, then moved the app to a temporary Trash. The probe used its own home, Applications directory, cache and synthetic user file. The user file was preserved. The SSH session was in `Background` without an isolated Aqua desktop, so the test covered the package lifecycle. Desktop launch and transfer from an Android device remain untested.

- [OpenMTP 3.2.25](https://github.com/ganeshrvel/openmtp/releases/tag/v3.2.25) shipped the 123,800,559-byte Apple Silicon DMG. Its measured SHA-256 is pinned. The downloaded file also matched the publisher’s [SHA-512 and size](https://github.com/ganeshrvel/openmtp/releases/download/v3.2.25/latest-mac.yml).
- [OpenMTP 3.3.0](https://github.com/ganeshrvel/openmtp/releases/tag/v3.3.0) shipped the 126,208,067-byte Apple Silicon DMG. Its SHA-256 matched the value published on GitHub.
- Both bundles use `io.ganeshrvel.openmtp` and Developer ID team `6UR4H85SA2`, include `arm64`, passed strict code-sign verification and were accepted by Gatekeeper as notarized Developer ID apps. [MIT license](https://github.com/ganeshrvel/openmtp/blob/v3.3.0/LICENSE).

[Native lifecycle](openmtp-native.log) · [3.2.25 preflight](openmtp-v3.2.25.log) · [3.3.0 preflight](openmtp-v3.3.0.log) · [Publisher SHA-512 check](openmtp-upstream-sha512.log).

Core crate tests finished with 161 passed and one ignored. Strict Clippy reports pre-existing findings in untouched code, including the cognitive complexity of `pick_asset` and `.unwrap()` in test modules. Formatting and patch checks passed.
