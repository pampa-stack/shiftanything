# Amethyst on Mac Apple Silicon

[Amethyst](https://github.com/ianyh/Amethyst) is an MIT-licensed window tiler. Both official universal ZIPs carry the Developer ID of Ian Ynda-Hummel (`82P2XLB4UH`) and report minimum macOS 12.0. Both passed ARM64, strict deep signing, notarization and Gatekeeper checks on a MacBook Air with macOS 26.6.2. [0.25.0 receipt](amethyst-v0.25.0.log) · [0.25.1 receipt](amethyst-v0.25.1.log)

The launcher installed 0.25.0 from the [official release](https://github.com/ianyh/Amethyst/releases/tag/v0.25.0), detected and applied [stable 0.25.1](https://github.com/ianyh/Amethyst/releases/tag/v0.25.1), then removed the updated bundle to the isolated profile's Trash. It downloaded both packages from GitHub and checked their published hashes and staged bundle signatures. The probe seeded version-list metadata from the official expanded assets. The synthetic user-data marker survived. App launch and Accessibility permissions were outside the SSH background test. [Native cycle log](amethyst-native.log)

To repeat on Apple Silicon, use the [isolated native probe](../pro-371-macos-overnight/README.md), save the two GitHub expanded asset pages and their preflight receipts as `amethyst-v0.25.0` and `amethyst-v0.25.1` with the probe's `-assets.html` and `.log` suffixes, then run `bash run-native.sh amethyst ianyh/Amethyst Amethyst v0.25.0 v0.25.1`.
