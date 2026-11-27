# MacBook Air package verification

MacBook Air with Apple Silicon, macOS 26.6.2, 27–28 November 2026. Baseline v0.20.8 (`1914684`) lists 96 verified Mac ARM64 recipes. This batch adds seventeen, bringing the total to 113.

| App | Official package versions | Apple signing team | Minimum macOS | Native result |
|---|---|---|---|---|
| [Moonlight](https://github.com/moonlight-stream/moonlight-qt/releases) | 6.1.0 → 6.2.0 | `45U78722YL` | 13.0 for the current package | [Install, update, remove](moonlight.log) |
| [OpenPencil](https://github.com/open-pencil/open-pencil/releases) | 0.16.0 → 0.17.0 | `N7D7M3Q3CD` | 13.0 | [Install, update, remove](openpencil.log) |
| [Sunshine](https://github.com/SunshineStream/Sunshine/releases) | 2026.906.222525 → 2026.914.233613 | `F8XQ7XCN2R` | 14.2 | [Install, update, remove](sunshine.log) |
| [AIRI](https://github.com/moeru-ai/airi/releases) | 0.15.2 → 0.15.4 | `433DLLA855` | 12.0 | [Install, update, remove](airi.log) |
| [OpenTTD](https://www.openttd.org/downloads/openttd-releases/latest) | 15.2 → 15.3 | `475CDUM4UL` | 11.0 on ARM64 | [Install, CLI launch, update, remove](openttd.log) |
| [Rectangle](https://github.com/rxhanson/Rectangle/releases) | 2.0.4 → 2.0.5, DMG | `XSYZ3E4B7D` | 14.0 | [Install, update, remove](rectangle.log) |
| [CodeEdit](https://github.com/CodeEditApp/CodeEdit/releases) | 0.3.5 → 0.3.6, DMG | `9VS2VKC5LQ` | 13.0 | [Install, update, remove](codeedit.log) |
| [Maccy](https://github.com/p0deje/Maccy/releases) | 2.7.1 → 2.8.0, ZIP | `MN3X4648SC` | 14.0 | [Install, update, remove](maccy.log) |
| [Blender](https://www.blender.org/releases/) | 5.2.5 LTS → 5.3.0, DMG | `68UA947AUU` | 13.0 for the current package | [Install, CLI launch, update, remove](blender.log) |
| [GIMP](https://www.gimp.org/downloads/) | 3.2.6 → 3.2.8, DMG | `T25BQ8HSJF` | 11.0 | [Install, CLI launch, update, remove](gimp.log) |
| [NetNewsWire](https://netnewswire.com/) | 7.3 → 7.3.1, ZIP | `M8L2WTLA8W` | 15.0 | [Install, update, remove](netnewswire.log) |
| [Krita](https://krita.org/en/download/) | 5.3.4.1 → 5.3.5, DMG | `DL93766A3G` | 11.0 on ARM64 | [Install, update, remove](krita.log) |
| [VLC](https://download.videolan.org/pub/videolan/vlc/last/macosx/) | 3.0.23 → 3.0.24, ARM64 DMG | `75GAHG3SZQ` | 11.0 | [Install, CLI launch, update, remove](vlc.log) |
| [Inkscape](https://inkscape.org/release/) | 1.4.3 → 1.4.4, ARM64 DMG | `SW3D6BB6A6` | 11.3 | [Install, CLI launch, update, remove](inkscape.log) |
| [Kdenlive](https://kdenlive.org/en/download/) | 26.08.2 → 26.08.3, ARM64 DMG | `5433B4KXM8` | 13.3 | [Install, update, remove](kdenlive.log) |
| [Subtitle Edit](https://github.com/SubtitleEdit/subtitleedit/releases) | 5.3.0 → 5.4.0, ARM64 DMG | `MFC9L2T4RB` | 12.0 | [Install, update, remove](subtitle-edit.log) |
| [OpenWhispr Local](https://github.com/OpenWhispr/openwhispr/releases) | 1.12.1 → 1.12.2, ARM64 DMG | `T832773L2J` | 12.0 | [Install, update, remove](openwhispr.log) |

All thirty-four bundles passed ARM64, strict signature verification and Gatekeeper notarization checks. Moonlight and Sunshine are GPL-3.0. OpenPencil, AIRI, CodeEdit, Maccy, NetNewsWire, Subtitle Edit, OpenWhispr and the free Rectangle edition are MIT. AIRI 0.15.5 was a prerelease, so the cycle follows the stable channel. OpenTTD is GPL-2.0-only. Blender, GIMP, Krita, VLC, Inkscape and Kdenlive use the GPL. The installer fetches the official packages directly. GitHub publishes digests for nineteen packages. [OpenTTD publishes the digest for 15.3](https://www.openttd.org/downloads/openttd-releases/latest). Blender and GIMP publish both digests in their respective [release archive](https://download.blender.org/release/) and [version index](https://www.gimp.org/gimp_versions.json). For Moonlight 6.1.0 and OpenTTD 15.2, the receipts pin digests computed from the official HTTPS archives after validating matching signing teams and notarization.

For the GitHub-backed recipes other than NetNewsWire, the historical release list was seeded from the official expanded assets because the public API responded HTTP 403. The installer cache started empty. The native backend selected and downloaded the old version, resolved the new stable version from GitHub, downloaded and applied it, recorded both versions and removed the app into the private Trash. The native backend resolves OpenTTD’s latest stable tag on GitHub, then downloads the reviewed DMG from the publisher’s CDN without redirects. Blender and GIMP each resolve their versions from the publisher's own endpoint with an empty release cache. GIMP verifies the published SHA-256 and permits downloads from HTTPS-only publisher mirrors. NetNewsWire resolves the publisher’s Mac ZIP link against the exact stable GitHub macOS release because the repository-wide latest pointer can return iOS. The cache started empty and the historical release used its reviewed digest. Krita reads the official download page and uses KDE’s published package digests. The file labeled 5.3.4.1 contains a signed bundle reporting 5.3.4. VLC reads the VideoLAN stable macOS directory and verifies both published SHA-256 digests. Stripping resource attributes broke the detached signature on `Contents/MacOS/plugins/plugins.dat`. The installer preserves those attributes only for the reviewed Apple Silicon VLC bundles and runs the same strict signature and Gatekeeper checks on the staged copy. Inkscape resolves the publisher’s stable redirect and verifies both SHA-256 digests published on inkscape.org. Kdenlive follows the KDE application release RSS and verifies each published digest from the KDE download archive. Its signed bundle stores the full patch version in `CFBundleVersion`. The launcher compares and displays the numeric version as 26.8.2 or 26.8.3. Both ran against their live publisher source from an empty release cache. All seventeen profiles preserved the synthetic data marker. OpenTTD, Blender, GIMP, VLC and Inkscape returned their expected versions through `openttd -h`, `Blender --version`, `gimp-3.2 --version`, `VLC -I dummy --no-plugins-cache --version` and `inkscape --version` with network and writes blocked. Graphical Open and first-run features were outside this headless SSH test. Rectangle Accessibility permissions, Maccy clipboard access and CodeEdit workspace access were left untouched. No OpenWhispr models, account, microphone or Accessibility grants were used. Its [local edition](https://github.com/OpenWhispr/openwhispr#free-vs-paid) is free to use and the official bundle also offers optional hosted services.

MonitorControl 4.4.0 passed preflight. The official 4.3.2 and 4.3.3 DMGs failed strict signature verification on the embedded Sparkle Updater because of Finder metadata, so the recipe remains disabled.

## Reproduce

On Mac ARM64 with Gatekeeper enabled, create a private `/private/tmp/shift-air-overnight.XXXXXXXX` folder and an empty `.shift-owned` marker there. Copy the three shell scripts and `native-cycle.rs` into it. Set `CARGO_TARGET_DIR` to its `target` folder and run `cargo build --locked -p artcraft-launcher-native --lib --message-format=json`, redirecting stdout to `recipe-build.jsonl`. Run `bash build-probe.sh`.

For each exact DMG URL, run `bash preflight.sh ID TAG ASSET URL BYTES SHA256` in the scratch folder. Capture the output as `ID-TAG.log`, using `open-pencil` for the preflight ID and `openpencil` for the recipe ID. For Maccy, match the exact published SHA-256 and size first, inspect the archive paths and in-bundle framework links, and expand the verified ZIP into a private folder. Apply the same ARM64, strict signature and Gatekeeper checks to `Maccy.app`. Record the verified SHA-256, byte count and official URL on the `ARTIFACT` line followed by `PASS PREFLIGHT maccy TAG` using the format in [the log](maccy.log).

Save each official GitHub `releases/expanded_assets/TAG` response as `RECIPE_ID-TAG-assets.html`. Run `bash run-native.sh RECIPE_ID REPO BUNDLE_NAME OLD_TAG NEW_TAG` with `Moonlight`, `OpenPencil`, `Sunshine`, `airi`, `OpenTTD`, `Rectangle`, `CodeEdit` or `Maccy` as the bundle name. For OpenTTD, save the official latest download page as `openttd-stable.html` instead of saving GitHub asset pages. The scripts attach images read-only and remove test files in their own scratch paths.

For Blender, save `blender-TAG.sha256` from the corresponding `BlenderMAJOR.MINOR/` directory in the official archive and run `bash run-native.sh blender blender/blender Blender 5.2.5 5.3.0`. The harness queries the publisher through the native backend.

For GIMP, save the publisher's `gimp_versions.json` response as `gimp-versions.json` and run `bash run-native.sh gimp GNOME/gimp GIMP 3.2.6 3.2.8`. The native backend reads the live index and preserves the separate Linux Flathub route.

For NetNewsWire, save the exact Mac release asset pages as `netnewswire-mac-7.3-assets.html` and `netnewswire-mac-7.3.1-assets.html`. Verify the official ZIPs as for Maccy and record the `ARTIFACT` and `PASS PREFLIGHT` lines. Run `bash run-native.sh netnewswire Ranchero-Software/NetNewsWire NetNewsWire mac-7.3 mac-7.3.1`. The ZIP reader discards the one genuine root AppleDouble metadata file and validates all remaining entries before extraction.

For Krita, save the official download page as `krita-download.html` and each package's `.mirrorlist` from KDE as `krita-TAG.mirrorlist`. Run `bash run-native.sh krita KDE/krita krita 5.3.4.1 5.3.5`. The harness starts with an empty release cache.

For VLC, save the stable download directory as `vlc-mac-stable.html` and the official digest for each ARM64 DMG as `vlc-TAG.sha256`. Run `bash run-native.sh vlc videolan/vlc VLC 3.0.23 3.0.24`. The harness starts with an empty release cache.

For Inkscape, save the official per-version digests as `inkscape-TAG.sha256` and the relative Location header from the stable release endpoint as `inkscape-stable-location.txt`. Run `bash run-native.sh inks inkscape/inkscape Inkscape 1.4.3 1.4.4`.

For Kdenlive, save the official package digests as `kdenlive-TAG.sha256` and the KDE application RSS as `kdenlive-releases.xml`. Run `bash run-native.sh kdenlive KDE/kdenlive kdenlive 26.08.2 26.08.3`.

For Subtitle Edit, save the official expanded asset pages as `subtitle-edit-v5.3.0-assets.html` and `subtitle-edit-v5.4.0-assets.html`. Run `bash run-native.sh subtitle-edit SubtitleEdit/subtitleedit "Subtitle Edit" v5.3.0 v5.4.0`.

For OpenWhispr, save the official expanded asset pages as `openwhispr-v1.12.1-assets.html` and `openwhispr-v1.12.2-assets.html`. Run `bash run-native.sh openwhispr OpenWhispr/openwhispr OpenWhispr v1.12.1 v1.12.2`.
