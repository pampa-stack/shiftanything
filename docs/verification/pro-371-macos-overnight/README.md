# MacBook Air package verification

MacBook Air with Apple Silicon, macOS 26.6.2, 27 November 2026. Baseline v0.20.8 (`1914684`) lists 96 verified Mac ARM64 recipes. This batch adds five.

| App | Official DMG versions | Apple signing team | Minimum macOS | Native result |
|---|---|---|---|---|
| [Moonlight](https://github.com/moonlight-stream/moonlight-qt/releases) | 6.1.0 → 6.2.0 | `45U78722YL` | 13.0 for the current package | [Install, update, remove](moonlight.log) |
| [OpenPencil](https://github.com/open-pencil/open-pencil/releases) | 0.16.0 → 0.17.0 | `N7D7M3Q3CD` | 13.0 | [Install, update, remove](openpencil.log) |
| [Sunshine](https://github.com/SunshineStream/Sunshine/releases) | 2026.906.222525 → 2026.914.233613 | `F8XQ7XCN2R` | 14.2 | [Install, update, remove](sunshine.log) |
| [AIRI](https://github.com/moeru-ai/airi/releases) | 0.15.2 → 0.15.4 | `433DLLA855` | 12.0 | [Install, update, remove](airi.log) |
| [OpenTTD](https://www.openttd.org/downloads/openttd-releases/latest) | 15.2 → 15.3 | `475CDUM4UL` | 11.0 on ARM64 | [Install, CLI launch, update, remove](openttd.log) |

All ten bundles passed ARM64, strict signature verification and Gatekeeper notarization checks. Moonlight and Sunshine are GPL-3.0. OpenPencil and AIRI are MIT. AIRI 0.15.5 was a prerelease, so the cycle follows the stable channel. OpenTTD is GPL-2.0-only. The installer fetches the official packages directly. GitHub publishes digests for seven packages. [OpenTTD publishes the digest for 15.3](https://www.openttd.org/downloads/openttd-releases/latest). For Moonlight 6.1.0 and OpenTTD 15.2, the receipts pin digests computed from the official HTTPS archives after validating matching signing teams and notarization.

The historical release list in each temporary profile was seeded from the official expanded assets because the public API responded HTTP 403. The installer cache started empty. The native backend selected and downloaded the old version, resolved the new stable version from GitHub, downloaded and applied it, recorded both versions and removed the app into the private Trash. The native backend resolves OpenTTD’s latest stable tag on GitHub, then downloads the reviewed DMG from the publisher’s CDN without redirects. All five profiles preserved the synthetic data marker. The installed OpenTTD binaries for both versions returned the expected version through `openttd -h` with network and writes blocked. Graphical Open and first-run features were outside this headless SSH test.

## Reproduce

On Mac ARM64 with Gatekeeper enabled, create a private `/private/tmp/shift-air-overnight.XXXXXXXX` folder and an empty `.shift-owned` marker there. Copy the three shell scripts and `native-cycle.rs` into it. Set `CARGO_TARGET_DIR` to its `target` folder and run `cargo build --locked -p artcraft-launcher-native --lib --message-format=json`, redirecting stdout to `recipe-build.jsonl`. Run `bash build-probe.sh`.

For each exact URL, run `bash preflight.sh ID TAG ASSET URL BYTES SHA256` in the scratch folder. Capture the output as `ID-TAG.log`, using `open-pencil` for the preflight ID and `openpencil` for the recipe ID. Save each official GitHub `releases/expanded_assets/TAG` response as `RECIPE_ID-TAG-assets.html`. Run `bash run-native.sh RECIPE_ID REPO BUNDLE_NAME OLD_TAG NEW_TAG` with `Moonlight`, `OpenPencil`, `Sunshine`, `airi` or `OpenTTD` as the bundle name. For OpenTTD, save the official latest download page as `openttd-stable.html` instead of saving GitHub asset pages. The scripts attach images read-only and remove test files in their own scratch paths.
