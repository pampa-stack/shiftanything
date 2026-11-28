# MacBook Air package verification

MacBook Air with Apple Silicon, macOS 26.6.2, 27–28 November 2026. Baseline v0.20.8 (`1914684`) lists 96 verified Mac ARM64 recipes. This batch adds eighteen, bringing the total to 114.

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
| [DeepChat](https://github.com/ThinkInAIXYZ/deepchat/releases) | 1.1.3 → 1.1.4, ARM64 DMG | `Y7P5QLKLYG` | 12.0 | [Install, update, remove](deepchat.log) |

All thirty-six bundles passed ARM64, strict signature verification and Gatekeeper notarization checks. Moonlight and Sunshine are GPL-3.0. OpenPencil, AIRI, CodeEdit, Maccy, NetNewsWire, Subtitle Edit, OpenWhispr and the free Rectangle edition are MIT. AIRI 0.15.5 was a prerelease, so the cycle follows the stable channel. OpenTTD is GPL-2.0-only. DeepChat is Apache-2.0. Blender, GIMP, Krita, VLC, Inkscape and Kdenlive use the GPL. The installer fetches the official packages directly. GitHub publishes digests for twenty-one packages. [OpenTTD publishes the digest for 15.3](https://www.openttd.org/downloads/openttd-releases/latest). Blender and GIMP publish both digests in their respective [release archive](https://download.blender.org/release/) and [version index](https://www.gimp.org/gimp_versions.json). For Moonlight 6.1.0 and OpenTTD 15.2, the receipts pin digests computed from the official HTTPS archives after validating matching signing teams and notarization.

For the GitHub-backed recipes other than NetNewsWire, the historical release list was seeded from the official expanded assets because the public API responded HTTP 403. The installer cache started empty. The native backend selected and downloaded the old version, resolved the new stable version from GitHub, downloaded and applied it, recorded both versions and removed the app into the private Trash. The native backend resolves OpenTTD’s latest stable tag on GitHub, then downloads the reviewed DMG from the publisher’s CDN without redirects. Blender and GIMP each resolve their versions from the publisher's own endpoint with an empty release cache. GIMP verifies the published SHA-256 and permits downloads from HTTPS-only publisher mirrors. NetNewsWire resolves the publisher’s Mac ZIP link against the exact stable GitHub macOS release because the repository-wide latest pointer can return iOS. The cache started empty and the historical release used its reviewed digest. Krita reads the official download page and uses KDE’s published package digests. The file labeled 5.3.4.1 contains a signed bundle reporting 5.3.4. VLC reads the VideoLAN stable macOS directory and verifies both published SHA-256 digests. Stripping resource attributes broke the detached signature on `Contents/MacOS/plugins/plugins.dat`. The installer preserves those attributes only for the reviewed Apple Silicon VLC bundles and runs the same strict signature and Gatekeeper checks on the staged copy. Inkscape resolves the publisher’s stable redirect and verifies both SHA-256 digests published on inkscape.org. Kdenlive follows the KDE application release RSS and verifies each published digest from the KDE download archive. Its signed bundle stores the full patch version in `CFBundleVersion`. The launcher compares and displays the numeric version as 26.8.2 or 26.8.3. Both ran against their live publisher source from an empty release cache. All eighteen profiles preserved the synthetic data marker. OpenTTD, Blender, GIMP, VLC and Inkscape returned their expected versions through `openttd -h`, `Blender --version`, `gimp-3.2 --version`, `VLC -I dummy --no-plugins-cache --version` and `inkscape --version` with network and writes blocked. Graphical Open and first-run features were outside this headless SSH test. Rectangle Accessibility permissions, Maccy clipboard access and CodeEdit workspace access were left untouched. No OpenWhispr models, account, microphone or Accessibility grants were used. Its [local edition](https://github.com/OpenWhispr/openwhispr#free-vs-paid) is free to use and the official bundle also offers optional hosted services.

MonitorControl 4.4.0 passed preflight. The official 4.3.2 and 4.3.3 DMGs failed strict signature verification on the embedded Sparkle Updater because of Finder metadata, so the recipe remains disabled.

### CotEditor

The [official 7.1.3 and 7.2.0 releases](https://github.com/coteditor/CotEditor/releases) provide notarized universal DMGs with signed bundle ID `com.coteditor.CotEditor`, team `HT3Z3A72WZ` and a macOS 26.0 minimum. Sh*ftAnything enables the recipe on Apple Silicon from macOS 26.0. Installation, update and removal passed on macOS 26.4 in an isolated profile, with the synthetic user data and private Trash checked. The result covers the package lifecycle. See the [native cycle log](coteditor.log).

The [license](https://github.com/coteditor/CotEditor/blob/7.2.0/LICENSE) covers source code under Apache-2.0 and image resources under CC BY-NC-ND 4.0. The launcher downloads the signed package from the publisher and the catalog uses a monogram.

### macshot Offline

On 28 November 2026, the native backend installed [4.3.0](https://github.com/sw33tLie/macshot/releases/tag/v4.3.0), updated to [4.4.0](https://github.com/sw33tLie/macshot/releases/tag/v4.4.0) and moved the app to the private Trash on Apple Silicon with macOS 26.4. Both official DMGs matched GitHub's size and SHA-256 and passed strict signature and notarization checks. The bundle is `com.sw33tlie.macshot.offline` and requires macOS 12.3. The recipe restricts the previous team `BR52ZWX528` to the reviewed 4.3.0 DMG and uses team `J4XGH8RFS8` for 4.4.0. The [upstream release notes](https://github.com/sw33tLie/macshot/blob/v4.4.0/README.md) describe the signature change and possible permission prompts. The synthetic profile marker survived the lifecycle. GUI launch and capture permissions remain untested. [Cycle](macshot.log) · [4.3.0 signature](macshot-v4.3.0.log) · [4.4.0 signature](macshot-v4.4.0.log)

In the isolated Tauri 0.20.9 app copied from the local DMG, the native inventory reported macshot 4.4.0 as installable at 8,624,563 bytes. The UI asked to approve an install requested through MCP, and clicking Rifiuta rejected it before a native job started. The catalog buttons were checked in Italian and English in Chromium using that inventory snapshot. The desktop approval window and the two catalog screenshots were inspected.

### Signal Desktop

On 28 November 2026, the native backend installed Signal Desktop 8.35.0, updated to 8.36.0 and removed it to the private Trash on Apple Silicon with macOS 26.4. Both official DMGs passed strict signature, notarization and Gatekeeper checks for team `U68MSDN6DR`. The recipe requires macOS 13.0 and preserves user data on update and removal. GUI launch and phone pairing remain untested. See the [native cycle](signal-desktop.log) and preflight logs for [8.35.0](signal-desktop-8.35.0.log) and [8.36.0](signal-desktop-8.36.0.log).

Signal's stable feed lists 269,301,286 bytes for 8.36.0. Its official CDN returned 269,303,849 bytes. The installer pins the exact size and SHA-256 of the independently verified, notarized CDN package. A new release can appear in the launcher once the feed reports it. Its installer becomes available after the corresponding package has been reviewed. [Official downloads](https://signal.org/download/) · [Stable macOS feed](https://updates.signal.org/desktop/latest-mac.yml)

### Firefox

On 28 November 2026, the native backend installed Firefox 160.0.1, updated it to 161.0 and removed it to a private Trash on Apple Silicon with macOS 26.4. The clean profile resolved the stable version from Mozilla. Both official DMGs matched Mozilla's published SHA-256 and passed strict signature, notarization and Gatekeeper checks for bundle `org.mozilla.firefox` and team `43AQ936H96`. The recipe uses Mozilla's en-US universal DMG on Apple Silicon with macOS 11.0 or later. The synthetic profile marker survived. GUI launch and Intel installation were outside this test. [Stable versions](https://product-details.mozilla.org/1.0/firefox_versions.json) · [Native cycle](firefox.log) · [160.0.1 preflight](firefox-160.0.1.log) · [161.0 preflight](firefox-161.0.log)

On 28 November 2026, the native backend installed Element 1.12.35, updated it to 1.12.36 and moved it to private Trash on Apple Silicon with macOS 26.4. The synthetic profile marker survived. The official HTTPS index identified both universal DMGs. The recorded SHA-256 digests were computed from the publisher’s downloads, with bundle `im.riot.app`, Developer ID team `7J4U792NQT`, strict signature verification and notarization checked on both releases. The managed route is for Apple Silicon with macOS 13 or later. GUI launch and Intel installation were outside the test. [Official index](https://packages.element.io/desktop/install/macos/index.html) · [Native cycle](element.log) · [1.12.35 preflight](element-1.12.35.log) · [1.12.36 preflight](element-1.12.36.log)

The isolated Tauri app reported Firefox 161.0 as installable at 167,399,384 bytes. Chromium rendered that native inventory and showed the enabled catalog action in both Italian and English. The stdio MCP client read the same inventory and rejected actions requiring an installed copy and requests with unknown IDs.

The combined Rust test run hit the existing intermittent PhotoCraft MCP fixture launch failure. The same fixture passed when rerun in isolation. The Firefox parser and install eligibility tests passed in the combined run.

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

For DeepChat, save the official expanded asset pages as `deepchat-v1.1.3-assets.html` and `deepchat-v1.1.4-assets.html`. Run `bash run-native.sh deepchat ThinkInAIXYZ/deepchat DeepChat v1.1.3 v1.1.4`.

For Dyad, Finicky and Hammerspoon, save each official GitHub expanded asset page and preflight log using the same names as for DeepChat. Run `bash run-native.sh dyad dyad-sh/dyad dyad v1.23.0 v1.24.0`, `bash run-native.sh finicky johnste/finicky Finicky v4.2.1 v4.2.2` and `bash run-native.sh hammerspoon Hammerspoon/hammerspoon Hammerspoon 1.1.0 1.1.1`.

For CotEditor, save the official expanded asset pages as `coteditor-7.1.3-assets.html` and `coteditor-7.2.0-assets.html`. Run `bash run-native.sh coteditor coteditor/CotEditor CotEditor 7.1.3 7.2.0`.

For macshot Offline, save the preflight logs and expanded assets as `macshot-v4.3.0.log`, `macshot-v4.4.0.log`, `macshot-v4.3.0-assets.html` and `macshot-v4.4.0-assets.html`. Run `bash run-native.sh macshot sw33tLie/macshot "macshot Offline" v4.3.0 v4.4.0`.

For Signal Desktop, save the verified DMG preflight logs as `signal-desktop-8.35.0.log` and `signal-desktop-8.36.0.log` and the live publisher feed as `signal-mac-stable.yml`. Run `bash run-native.sh signal-desktop signalapp/Signal-Desktop Signal 8.35.0 8.36.0`. Keep the scratch folder empty of prior release caches so the harness queries the live feed.

For Firefox, save Mozilla's stable index as `firefox-versions.json` and the `SHA256SUMS` from each version directory in Mozilla's archive as `firefox-TAG.sha256`. Pass the quoted internal filename `Firefox TAG.dmg` and the URL containing `Firefox%20TAG.dmg` to the preflight script. Run `bash run-native.sh firefox mozilla-firefox/firefox Firefox 160.0.1 161.0`.

For Element, save the official macOS package index as `element-index.html`. Preflight the two exact universal DMG links and run `bash run-native.sh element element-hq/element-web Element 1.12.35 1.12.36`.
