# MacBook Air package verification

MacBook Air with Apple Silicon, macOS 26.6.2, 27 November 2026. Baseline v0.20.8 (`1914684`) lists 96 verified Mac ARM64 recipes. This batch adds three.

| App | Official DMG versions | Apple signing team | Minimum macOS | Native result |
|---|---|---|---|---|
| [Moonlight](https://github.com/moonlight-stream/moonlight-qt/releases) | 6.1.0 → 6.2.0 | `45U78722YL` | 13.0 for the current package | [Install, update, remove](moonlight.log) |
| [OpenPencil](https://github.com/open-pencil/open-pencil/releases) | 0.16.0 → 0.17.0 | `N7D7M3Q3CD` | 13.0 | [Install, update, remove](openpencil.log) |
| [Sunshine](https://github.com/SunshineStream/Sunshine/releases) | 2026.906.222525 → 2026.914.233613 | `F8XQ7XCN2R` | 14.2 | [Install, update, remove](sunshine.log) |

All six bundles passed ARM64, strict signature verification and Gatekeeper notarization checks. Moonlight and Sunshine are GPL-3.0. OpenPencil is MIT. The installer fetches official packages directly. GitHub publishes digests for five packages. Moonlight 6.1.0 has no published GitHub digest, so the receipt pins the digest computed from the official HTTPS download after validating its matching signing team and notarization.

The historical release list in each temporary profile was seeded from the official expanded assets because the public API responded HTTP 403. The installer cache started empty. The native backend selected and downloaded the old version, resolved the new stable version from GitHub, downloaded and applied it, recorded both versions and removed the app into the private Trash. All three profiles preserved the synthetic data marker. The headless SSH session did not exercise the graphical Open command or first-run features.

## Reproduce

On Mac ARM64 with Gatekeeper enabled, create a private `/private/tmp/shift-air-overnight.XXXXXXXX` folder and an empty `.shift-owned` marker there. Copy the three shell scripts and `native-cycle.rs` into it. Set `CARGO_TARGET_DIR` to its `target` folder and run `cargo build --locked -p artcraft-launcher-native --lib --message-format=json`, redirecting stdout to `recipe-build.jsonl`. Run `bash build-probe.sh`.

For each exact URL, run `bash preflight.sh ID TAG ASSET URL BYTES SHA256` in the scratch folder. Capture the output as `ID-TAG.log`, using `open-pencil` for the preflight ID and `openpencil` for the recipe ID. Save each official GitHub `releases/expanded_assets/TAG` response as `RECIPE_ID-TAG-assets.html`. Run `bash run-native.sh RECIPE_ID REPO BUNDLE_NAME OLD_TAG NEW_TAG` with `Moonlight`, `OpenPencil` or `Sunshine` as the bundle name. The scripts attach images read-only and remove test files in their own scratch paths.
