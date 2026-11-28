# Freeplane and Deskflow on macOS Apple Silicon

November 28, 2026. MacBook Air, macOS 26.6.2, `arm64`, Gatekeeper enabled. The SSH session was `Background` without an isolated Aqua desktop. No GUI was started on the personal desktop.

## Freeplane

The official [1.13.3](https://github.com/freeplane/freeplane/releases/tag/release-1.13.3) and [1.13.5](https://github.com/freeplane/freeplane/releases/tag/release-1.13.5) Apple Silicon DMGs are 144,017,558 and 147,343,258 bytes. Both match the publishers’ GitHub SHA-256 values. The `org.freeplane.launcher` bundles signed by Developer ID team `CSHVD99Y7K` pass strict verification and Gatekeeper notarization checks. The [1.13.4 build](https://github.com/freeplane/freeplane/releases/tag/release-1.13.4) matches its published SHA-256 and is signed by the same team but fails the notarization requirement. The native recipe permits only the two versions used in the cycle.

`NativeBackend` installed 1.13.3, detected and updated it to 1.13.5, then removed it to a temporary Trash. The test verified the product receipt, downloaded bytes, bundle location and preservation of a synthetic user file. [Native cycle](freeplane-native.log) · [1.13.3 signed](freeplane-release-1.13.3.log) · [1.13.4 rejected](freeplane-release-1.13.4.log) · [1.13.5 signed](freeplane-release-1.13.5.log).

Core test results were 162 passed and one ignored, including explicit rejection of the 1.13.4 package. Strict Clippy still reports the existing complexity finding in `pick_asset`, which this change leaves intact.

## Deskflow

The official [1.26.0](https://github.com/deskflow/deskflow/releases/tag/v1.26.0) and [1.27.0](https://github.com/deskflow/deskflow/releases/tag/v1.27.0) Apple Silicon DMGs downloaded and matched GitHub’s SHA-256. Both read-only noninteractive mount attempts returned `hdiutil: attach canceled`. `hdiutil imageinfo` on the downloaded 1.27.0 image reported `Software License Agreement: true`. The official macOS release lists only DMG packages. Desktop installation via the vendor package remains manual. [1.26.0 attempt](deskflow-v1.26.0.log) · [1.27.0 attempt](deskflow-v1.27.0.log).
