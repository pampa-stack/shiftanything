# DockDoor on Mac Apple Silicon

[DockDoor](https://dockdoor.net/) is free window-preview software under [GPL-3.0](https://github.com/ejbills/DockDoor/blob/main/LICENSE). Both official universal DMGs carry the Developer ID of Ethan Bills (`2Q775S63Q3`) and require macOS 13.0. Both passed ARM64, strict deep signing, notarization and Gatekeeper checks on a MacBook Air with macOS 26.6.2. [1.42.4 receipt](dockdoor-1.42.4.log) · [1.42.5 receipt](dockdoor-1.42.5.log)

The launcher downloaded and installed [1.42.4](https://github.com/ejbills/DockDoor/releases/tag/1.42.4), detected and applied [1.42.5](https://github.com/ejbills/DockDoor/releases/tag/1.42.5), then moved the updated bundle to the isolated profile's Trash. It validated the published SHA-256 digests and staged bundle signatures. The probe seeded version-list metadata from the official expanded assets. The synthetic data marker survived. App launch, Accessibility and Screen Recording were outside this SSH background test. [Native cycle log](dockdoor-native.log)

To repeat on Apple Silicon, follow the [isolated native probe](../pro-371-macos-overnight/README.md). Save the official expanded asset pages as `dockdoor-1.42.4-assets.html` and `dockdoor-1.42.5-assets.html`, run `preflight.sh` for both official DMGs, then run `bash run-native.sh dockdoor ejbills/DockDoor DockDoor 1.42.4 1.42.5`.
