# MacVim on Mac Apple Silicon

[MacVim](https://macvim.org/) is a macOS editor distributed under the [Vim license](https://github.com/macvim-dev/macvim/blob/master/LICENSE). Both reviewed official DMGs contain the universal `org.vim.MacVim` bundle, signed by Yee Cheng Chin (`WG3S88DD2E`). The ARM64 slice reports minimum macOS 11.0. Both packages passed published SHA-256, strict deep signing, Gatekeeper and notarization checks on a MacBook Air with macOS 26.6.2. [r183 receipt](macvim-release-183.log) · [r184 receipt](macvim-release-184.log)

The isolated launcher probe downloaded and installed [r183](https://github.com/macvim-dev/macvim/releases/tag/release-183) and detected bundle version `9.2.321`. It downloaded and applied [r184](https://github.com/macvim-dev/macvim/releases/tag/release-184), detected `9.2.1252`, then moved the updated app to the temporary profile's Trash. The synthetic user-data marker survived. Release-list metadata came from the official expanded assets. The SSH background test did not exercise opening the app. [Native cycle log](macvim-native.log)

To repeat on Apple Silicon, use the [isolated native probe](../pro-371-macos-overnight/README.md) with the two expanded asset pages and preflight logs. Then run `bash run-native.sh macvim macvim-dev/macvim MacVim release-183 release-184`.
