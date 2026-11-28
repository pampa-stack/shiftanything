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

### Itsycal

On 28 November 2026, the native backend followed the [publisher’s appcast](https://s3.amazonaws.com/itsycal/itsycal.xml), installed Itsycal 0.15.14, updated it to 0.15.15 and removed it on Apple Silicon with macOS 26.4. Both downloads started from an empty package cache. The two [official ZIPs](https://www.mowglii.com/itsycal/versionhistory) passed strict signature and Gatekeeper notarization checks with bundle ID `com.mowglii.ItsycalApp` and team `HFT3T55WND`. The recipe pins the SHA-256 computed from each package and checks the matching publisher metadata. The synthetic data marker survived and the bundle reached the private Trash. This test covers install, update and remove on Apple Silicon. [Native cycle](itsycal.log) · [0.15.14 preflight](itsycal-0.15.14.log) · [0.15.15 preflight](itsycal-0.15.15.log)

The appcast also lists the 0.29.0 ARM64 package for macOS 27. Its signature and notarization passed preflight. On macOS 27 the launcher reports 0.29.0 without an install action until that package passes the native cycle. Intel installation and GUI launch are outside this evidence. [0.29.0 preflight](itsycal-0.29.0.log)

### Sloth

On 28 November 2026, the native backend installed [Sloth 3.5](https://github.com/sveinbjornt/Sloth/releases/tag/3.5), updated it to [3.6](https://github.com/sveinbjornt/Sloth/releases/tag/3.6) and removed it in an isolated MacBook Air Apple Silicon profile. Both official ZIPs matched their published SHA-256, contained ARM64 binaries, and passed strict code signature and notarization checks with team `5WX26Y89JP`. The archive extractor used its existing framework link validation for Sparkle. The cycle preserved a synthetic data file and sent the bundle to the private Trash. The test did not exercise the GUI. [3.5 preflight](sloth-3.5.log) · [3.6 preflight](sloth-3.6.log) · [Cycle log](sloth-native.log)

### Craft Anything

On 28 November 2026, the native backend installed [Craft Anything 0.6.0](https://github.com/storytold/craft-anything/releases/tag/v0.6.0), detected [0.6.1](https://github.com/storytold/craft-anything/releases/tag/v0.6.1) through the ArtCraft release feed, updated it and moved the updated app to the private Trash. The Apple Silicon test ran on macOS 26.4. Both downloads matched the published SHA-256 and passed strict signature, architecture and notarization checks for bundle `ai.storyteller.craft-anything`, team `DJ6XS33FX8`. The synthetic user data survived. The macOS recipe selects the reviewed DMGs and requires macOS 11.0. This proof covers the package lifecycle. See [cycle](craft-anything.log), [0.6.0 preflight](craft-anything-v0.6.0.log) and [0.6.1 preflight](craft-anything-v0.6.1.log). The Linux GUI and package cycle are documented in [the Linux proof](../pro-371-linux-appimage-four/README.md).

### Stellarium

The two official [26.2](https://github.com/Stellarium/stellarium/releases/tag/v26.2) and [26.3](https://github.com/Stellarium/stellarium/releases/tag/v26.3) ZIPs contain universal bundles signed by team `3WW8XR23DF`. Both match their published SHA-256 and passed deep, strict verification and Gatekeeper on Apple Silicon. Their ARM64 slices specify macOS 12.0. The current archive contains 4,473 entries and 877,976,263 unpacked bytes. The recipe pins both artifacts and limits the two library aliases to their documented destinations inside the bundle. [26.2 signature](stellarium-v26.2.log) · [26.3 signature](stellarium-v26.3.log) · [26.2 ZIP layout](stellarium-v26.2-layout.log) · [26.3 ZIP layout](stellarium-v26.3-layout.log)

On 28 November 2026, the native backend installed 26.2, updated it to 26.3 and moved it to the test profile’s Trash on Apple Silicon with macOS 26.4. The old release metadata came from GitHub’s expanded asset page. The package cache started empty and the backend downloaded both official ZIPs and verified their published digests. An earlier attempt exceeded the 60-second Gatekeeper timeout. The recipe now allows up to 180 seconds for that assessment while retaining the same signature and notarization requirements. The synthetic data marker survived. This run covers install, update and remove. [Native cycle](stellarium.log)

### Floorp

On 28 November 2026, the native backend installed [Floorp 12.21.1](https://github.com/Floorp-Projects/Floorp/releases/tag/v12.21.1), updated it to [12.21.2](https://github.com/Floorp-Projects/Floorp/releases/tag/v12.21.2) and removed it on Apple Silicon with macOS 26.4. The official DMGs matched GitHub's SHA-256 and passed the architecture, Developer ID and Gatekeeper notarization checks for bundle `app.floorp.Floorp`, team `DV2U35YBHT`. Detection reads the Floorp version before the Firefox engine version in the signed bundle. The synthetic data marker survived and the bundle reached the temporary Trash. This run covered neither GUI launch nor the Intel slice of the universal DMG. The separate Linux Flathub route is covered by the backend regression test. [Native cycle](floorp.log) · [12.21.1 preflight](floorp-v12.21.1.log) · [12.21.2 preflight](floorp-v12.21.2.log)

### GnuCash

On 28 November 2026, the native backend downloaded and installed the [official GnuCash 5.16 DMG](https://github.com/Gnucash/gnucash/releases/tag/5.16), updated it to [5.17](https://github.com/Gnucash/gnucash/releases/tag/5.17) and moved it to the private Trash on Apple Silicon with macOS 26.4. Both archives matched the GitHub SHA-256 and passed strict signature and notarization checks for bundle `org.gnucash.Gnucash`, team `Y9EHT5WMK7`. The installed versions `5.16-3` and `5.17-1` map to the upstream release versions. The profile and downloads were stored on T7, and the synthetic data marker survived. GnuCash is GPL-2.0-or-later. GUI launch and Intel installation were outside the test. [Native cycle](gnucash.log) · [5.16 preflight](gnucash-5.16.log) · [5.17 preflight](gnucash-5.17.log)

The Tauri binary reported an installable GnuCash 5.17 with a 180,289,229-byte download in a fresh profile. Chromium rendered that native snapshot and showed the enabled catalog button and macOS download size in [Italian](gnucash-catalog-it.png) and [English](gnucash-catalog-en.png). This catalog check did not fetch preview media.

### MonitorControl

On 28 November 2026, the native backend installed [MonitorControl 4.4.0](https://github.com/MonitorControl/MonitorControl/releases/tag/v4.4.0) and moved it to the private Trash on Apple Silicon with macOS 26.4. It also updated an isolated copy of [4.2.0](https://github.com/MonitorControl/MonitorControl/releases/tag/v4.2.0) to 4.4.0 and removed the updated bundle. The synthetic profile marker survived. Both DMGs passed strict signature and Gatekeeper notarization checks. The 4.4.0 download matched GitHub's published SHA-256 and size. The current Apple Silicon recipe requires macOS 11.0 and uses bundle `app.monitorcontrol.MonitorControl` with team `299YSU96J7`.

The historical 4.2.0 DMG uses bundle `me.guillaumeb.MonitorControl` and team `CYC8C8R4K9`. The harness verifies it before seeding the isolated update profile. Only the reviewed 4.4.0 DMG is selectable from the launcher. The 4.3.2 and 4.3.3 DMGs failed strict signature verification on the embedded Sparkle Updater. GUI launch, monitor access and Intel installation were outside the test. [Native cycle](monitorcontrol.log) · [4.2.0 preflight](monitorcontrol-v4.2.0.log) · [4.4.0 preflight](monitorcontrol-v4.4.0.log)

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

### Ranked Apple Silicon preflight

On the MacBook Air, Nuclear matched the publisher’s SHA-256 for both [1.54.2](https://github.com/nukeop/nuclear/releases/tag/player%401.54.2) and [1.56.4](https://github.com/nukeop/nuclear/releases/tag/player%401.56.4). Both DMGs mounted read-only and contained an ARM64 bundle with no Developer ID team. Strict signature verification passed and Apple’s notarization requirement failed on both. The current release receipt defers the Apple Silicon package. The older package measured 23,545,343 bytes with SHA-256 `d0a0b03a5aa601b4f0a5a8613e423cc6047515e274edf3a53ca35d2d3a284337`.

On 28 November 2026, four official Apple Silicon DMGs matched their published SHA-256 and mounted read-only from the internal disk. CopyQ 17.0.0 and Xournal++ 1.3.8 failed deep, strict signature verification. Filee 2.5.0 and darktable 5.6.2 passed that step and failed the notarization requirement. Those packages left the verification count unchanged. The [package receipts](ranked-preflight.json) record the exact asset, digest, size and result. Initial attempts from the external disk stopped at the mount and did not establish the package result.

`python3 tools/ordina_cicli.py macos-arm64` moves a candidate to `deferred_preflight` when each of its cataloged package sources for that architecture has a matching failed preflight. A different release URL or an additional route moves the candidate back into the schedule. Compare the digest in the receipt before retesting a replacement asset under the same release URL.

[Logseq 0.10.15](https://github.com/logseq/logseq/releases/tag/0.10.15) and [Logseq OG 1.0.0](https://github.com/logseq/og/releases/tag/1.0.0) each passed ARM64, integrity and notarization checks from the internal disk. Their packages use different bundle IDs and signing teams. The 1.0.0 page identifies its package as beta, and the original repository's current release is the separate database edition. This preflight leaves the current Linux route and the verified Mac list unchanged. [Prior macOS file-graph cycle](../pro-371-macos-next-batch.md#logseq-file-graphs)

On 28 November 2026, the official OpenRefine [3.9.5](https://github.com/OpenRefine/OpenRefine/releases/tag/3.9.5) and [3.10.1](https://github.com/OpenRefine/OpenRefine/releases/tag/3.10.1) DMGs matched their published SHA-256 on the Air. Both packages contain an Intel-only `JavaAppLauncher` executable. The 3.9.5 DMG is 183,100,879 bytes and its SHA-256 is `54b41c4e61bc1f2c9ceb55315b4a4e0f7f1a25ae22e87d1f7949b64343591f36`. The [3.10.1 receipt](ranked-preflight.json) keeps that release out of the Apple Silicon queue. The publisher tracks [Apple Silicon support](https://forum.openrefine.org/t/future-openrefine-compatibility-with-apple-silicon/2994) separately.


On 28 November 2026, two official [QuickRecorder](https://github.com/lihaoyun6/QuickRecorder/releases) DMGs, 1.6.8 and 1.6.9, matched the published SHA-256 and contained ARM64 bundles. Both used an Apple Development identity and Gatekeeper rejected them. The current [RapidRAW DMG](https://github.com/CyberTimon/RapidRAW/releases/tag/v1.7.1) matched the published digest and contained an ARM64 bundle with a valid ad hoc signature. Gatekeeper rejected it. The Mac receipts leave the separate RapidRAW Linux recipe available.

[Mixxx](https://mixxx.org/download/) 2.5.4 and 2.5.6 use GPL-2.0-or-later. Both official ARM64 DMGs matched the publisher's checksums and the app bundles passed strict signature and notarization checks with team `JBLRSP95FC`. The native disk-image attach returned `attach canceled` with closed stdin. An interactive read-only mount confirmed the disk image license prompt. The [Homebrew cask](https://formulae.brew.sh/api/cask/mixxx.json) points to that same ARM64 image and digest. These checks ended before installation and retain the exact package receipts for a future user-consent flow.

[UltiMaker Cura](https://github.com/Ultimaker/Cura) uses LGPL-3.0 for its code. The [5.13.0](https://github.com/Ultimaker/Cura/releases/tag/5.13.0) and [5.14.0](https://github.com/Ultimaker/Cura/releases/tag/5.14.0) ARM64 DMGs matched the published SHA-256 and passed strict signature and Gatekeeper checks. The bundle identifier is `nl.ultimaker.cura_UltiMaker_Cura` and the Developer ID team is `V4B3JXRRQS`. On the Air, the native backend downloaded the older DMG into a private profile and stopped at `hdiutil: attach canceled`. Both releases present an embedded license before mounting. The [Homebrew cask](https://formulae.brew.sh/api/cask/ultimaker-cura.json) references the same current ARM64 image and digest. Both Mac sources are deferred until the launcher can show and accept the license with user consent. The tested Linux AppImage is still eligible.

### Raspberry Pi Imager

On 28 November 2026, the MacBook Air matched the published SHA-256 for both [2.0.11](https://github.com/raspberrypi/rpi-imager/releases/tag/v2.0.11) and [2.0.12](https://github.com/raspberrypi/rpi-imager/releases/tag/v2.0.12). The Universal bundles contain ARM64 and require macOS 13. The signing team is `8RDZTRXE62` and the bundle ID is `com.raspberrypi.rpi-imager`. Both passed deep, strict signature verification and Gatekeeper reported `Notarized Developer ID`. The 2.0.11 download was 51,896,529 bytes and 2.0.12 was 36,609,779 bytes. [2.0.11 preflight](raspberry-pi-imager-v2.0.11.log) · [2.0.12 preflight](raspberry-pi-imager-v2.0.12.log)

The official stable URL resolved to 2.0.12. From a temporary profile and the publisher's release index, NativeBackend downloaded and installed 2.0.11, updated it to 2.0.12, verified the managed version, and removed it to the profile's Trash. The synthetic profile marker survived. The test covers package management on Apple Silicon. GUI launch and writing a disk image remain untested. [Native cycle log](raspberry-pi-imager-native.log)

### OpenRA and xbar package inspection

On 28 November 2026 the Air inspected the official OpenRA [20250303](https://github.com/OpenRA/OpenRA/releases/tag/release-20250303) and [20250330](https://www.openra.net/download/) DMGs. Each contains Dune 2000, Red Alert and Tiberian Dawn as separate app bundles. All six contain ARM64 and passed strict signature verification and Gatekeeper notarization with Developer ID team `XT33TSN4G6`. The current digest matches the [Homebrew cask](https://formulae.brew.sh/cask/openra). The native installer stages one named app bundle per catalog entry. The macOS catalog needs an explicit way to choose among the three games before enabling installation. [20250303 package](openra-release-20250303.log) · [20250330 package](openra-release-20250330.log)

The official [xbar 2.1.6-beta and 2.1.7-beta](https://github.com/matryer/xbar/releases) DMGs each contain a universal app with bundle ID `com.xbarapp.app`, signed by team `N3H5B92L5N`. Both passed strict signature and Gatekeeper checks. The current 2.1.7-beta digest matches the [Homebrew cask](https://formulae.brew.sh/cask/xbar). The latest release is a beta published in October 2021. These were read-only checks on the disk image. A lifecycle test still needs an isolated plugin directory for xbar. The verified install list remains unchanged for both apps. [2.1.6-beta package](xbar-v2.1.6-beta.log) · [2.1.7-beta package](xbar-v2.1.7-beta.log)

### Mister Download Manager package inspection

On 28 November 2026 the MacBook Air inspected the experimental Apple Silicon DMGs for [0.5.0](https://github.com/anefzaoui/misterdownloadmanager/releases/tag/v0.5.0) and [0.5.2](https://github.com/anefzaoui/misterdownloadmanager/releases/tag/v0.5.2). Both measured sizes and SHA-256 digests match GitHub. Each contains an ARM64 bundle with ID `io.github.anefzaoui.misterdownloadmanager` and minimum macOS 11.0. Both report no Developer ID team and fail the notarization requirement. The verified Apple Silicon list stays unchanged. [0.5.0 package](mister-download-manager-v0.5.0.log) · [0.5.2 package](mister-download-manager-v0.5.2.log) · [Verified Linux cycle](../pro-371-linux-mister-download-manager/README.md)

## Reproduce

On Mac ARM64 with Gatekeeper enabled, create a private `/private/tmp/shift-air-overnight.XXXXXXXX` folder and an empty `.shift-owned` marker there. If the internal disk is below the harness floor, the native runner also accepts a private `/Volumes/T7-Backup/shift-air-overnight.XXXXXXXX` folder. Copy the three shell scripts and `native-cycle.rs` into it. Set `CARGO_TARGET_DIR` to its `target` folder and run `cargo build --locked -p artcraft-launcher-native --lib --message-format=json`, redirecting stdout to `recipe-build.jsonl`. Run `bash build-probe.sh`.

For each exact DMG URL, run `bash preflight.sh ID TAG ASSET URL BYTES SHA256` in the scratch folder. Capture the output as `ID-TAG.log`, using `open-pencil` for the preflight ID and `openpencil` for the recipe ID. For Maccy, match the exact published SHA-256 and size first, inspect the archive paths and in-bundle framework links, and expand the verified ZIP into a private folder. Apply the same ARM64, strict signature and Gatekeeper checks to `Maccy.app`. Record the verified SHA-256, byte count and official URL on the `ARTIFACT` line followed by `PASS PREFLIGHT maccy TAG` using the format in [the log](maccy.log).

Save each official GitHub `releases/expanded_assets/TAG` response as `RECIPE_ID-TAG-assets.html`. Run `bash run-native.sh RECIPE_ID REPO BUNDLE_NAME OLD_TAG NEW_TAG` with `Moonlight`, `OpenPencil`, `Sunshine`, `airi`, `OpenTTD`, `Rectangle`, `CodeEdit` or `Maccy` as the bundle name. For OpenTTD, save the official latest download page as `openttd-stable.html` instead of saving GitHub asset pages. The scripts attach images read-only and remove test files in their own scratch paths.

For Itsycal, save the official appcast as `itsycal.xml` and verify both ZIPs from the publisher’s version history. Record the exact SHA-256 and size, ARM64, the bundle ID and version, the strict signature and Gatekeeper notarization in each preflight log. Run `bash run-native.sh itsycal sfsam/Itsycal Itsycal 0.15.14 0.15.15` on a compatible macOS release.

For Stellarium, save the official GitHub expanded asset pages and preflight logs for both Qt 6 macOS ZIPs. After matching the published SHA-256, extract each archive into a private directory and verify ARM64, the bundle version, the strict signature, and Gatekeeper notarization. Run `bash run-native.sh stellarium Stellarium/stellarium Stellarium v26.2 v26.3`.

For Floorp, save the official GitHub expanded asset pages and the preflight logs for both universal DMGs. Run `bash run-native.sh floorp Floorp-Projects/Floorp Floorp v12.21.1 v12.21.2`.

For GnuCash, save the official GitHub asset pages and verify the two Apple Silicon DMGs. Run `bash run-native.sh gnucash Gnucash/gnucash Gnucash 5.16 5.17`.

For MonitorControl, save the GitHub asset pages for both tags and use `bash run-native.sh monitorcontrol MonitorControl/MonitorControl MonitorControl v4.2.0 v4.4.0`. The harness independently verifies the pinned 4.2.0 archive before copying the signed fixture into the private profile.

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

For Craft Anything, save the official GitHub expanded asset pages and the matching preflight logs as `craft-anything-v0.6.{0,1}-assets.html` and `craft-anything-v0.6.{0,1}.log`. Run `bash run-native.sh craft-anything storytold/craft-anything "Craft Anything" v0.6.0 v0.6.1`.


For Sloth, save the published GitHub expanded assets as `sloth-3.5-assets.html` and `sloth-3.6-assets.html`. Verify size, SHA-256, bundle identifier, version, ARM64 slice, signing identity and notarization for each ZIP, then save the preflight output as `sloth-3.5.log` and `sloth-3.6.log`. Execute `bash run-native.sh sloth sveinbjornt/Sloth Sloth 3.5 3.6` in the dedicated scratch directory with the native cycle binary.

The GPT4All v3.9.0 and v3.10.0 macOS DMGs both mounted read-only. Each contained `gpt4all-installer-darwin.app`, an Intel-only installer with bundle ID `com.yourcompany.installerbase` and version `4.8.1`. The Apple Silicon direct-copy recipe requires the application bundle, so this check stopped before running the installer and added no verified cycle. The latest package receipt records the downloaded file's measured size and SHA-256.

The official SumatraPDF macOS arm64 DMGs for [26.11.20.1](https://www.sumatrapdfreader.org/dlng/mac/26.11.20.1/SumatraPDF-mac-arm64.dmg) and [26.11.24.2](https://www.sumatrapdfreader.org/dlng/mac/26.11.24.2/SumatraPDF-mac-arm64.dmg) passed strict signature and notarization checks. The bundle identifier was `org.sumatrapdfreader.SumatraPDF` and the signing team was `2LGSCEWRR9`. Both executables declared minimum macOS 27.0 in `LC_BUILD_VERSION`, while the Air was running 26.6.2. The test stopped at package inspection and left the verified cycle count unchanged. The measured SHA-256 values were `5c402230ec0615015966244215be24fa6d67ea86ef9f7bce576ad68af5fa9361` (17,566,410 bytes) and `52f11767dbe66ed505e966893a99d5591abba2ed4ff6e7ef75a49a3b20199b56` (18,752,303 bytes), respectively.

All three Apple Silicon ZIPs for [mpv 0.42.0](https://github.com/mpv-player/mpv/releases/expanded_assets/v0.42.0) matched the published SHA-256 and contained `mpv.app` in a nested tarball. The arm64 binaries require macOS 15.0, 26.0 and 27.2 respectively. Each had an ad hoc signature with no Developer ID team. Strict verification flagged the unsigned `Contents/MacOS/.gitkeep` subcomponent, and Gatekeeper reported no usable signature. The launcher defers the Apple Silicon recipe. The ZIPs for macOS 15 and 27 measured 48,835,385 bytes (`be1b8d2445e6880860869c080b8710dee189721f1fa4689840db6a2c10d7be69`) and 47,876,256 bytes (`87c7af5d908eb85217899b57d9a24cc9f5316daa93bb6b4a494a34cea25330b7`). The receipt records the macOS 26 build tested on the Air.

The [FontForge 20251009](https://github.com/fontforge/fontforge/releases/expanded_assets/20251009) DMG contained the Intel executable `Resources/opt/local/bin/fontforge`. Its SHA-256 matched the published digest and `lipo -verify_arch arm64` rejected the executable. The [FreeTube 0.26.0 beta](https://github.com/FreeTubeApp/FreeTube/releases/tag/v0.26.0-beta) DMG contained an arm64 executable, with no Developer ID team, and failed the notarization requirement. Both checks stopped before installation and left the verified Apple Silicon count unchanged.

## Jellyfin Desktop

The official Apple Silicon DMGs for Jellyfin Media Player 1.11.1 and 1.12.0 passed strict signature verification and Gatekeeper notarization on the Air (macOS 26.6.2). Both contained the arm64 bundle `tv.jellyfin.player`, signed by Anthony Lavado (`6YC6EAV7ES`). The launcher enforces the current release’s documented minimum of macOS 13.0. Jellyfin Desktop is GPL-2.0. [Release 1.11.1](https://github.com/jellyfin/jellyfin-desktop/releases/tag/v1.11.1) · [Release 1.12.0](https://github.com/jellyfin/jellyfin-desktop/releases/tag/v1.12.0)

The native backend installed 1.11.1, updated it to 1.12.0 and removed the bundle into the private Trash while retaining synthetic data. The two asset receipts record the downloaded sizes and measured SHA-256 values. The GUI was outside the scope of the background SSH test. [Cycle](jellyfin-desktop-native.log) · [Previous asset](jellyfin-desktop-v1.11.1.log) · [Current asset](jellyfin-desktop-v1.12.0.log)

Save the two official GitHub expanded asset pages as `jellyfin-desktop-v1.11.1-assets.html` and `jellyfin-desktop-v1.12.0-assets.html` and reproduce each asset receipt with `preflight.sh`. Run `bash run-native.sh jellyfin-desktop jellyfin/jellyfin-desktop "Jellyfin Media Player" v1.11.1 v1.12.0`.

## Ruffle and AionUi package checks

On the Air (macOS 26.6.2), all five packages passed strict signature checks and Gatekeeper reported `Notarized Developer ID`. The package inspections did not launch the apps or run the launcher lifecycle.

- Ruffle's 0.7.0, 0.7.1 and 0.8.0 archives contained the signed universal bundle `rs.ruffle.ruffle`, published by Ruffle LLC (`H22337M38N`). The arm64 slice requires macOS 11.0. [0.7.0](ruffle-v0.7.0.log) · [0.7.1](ruffle-v0.7.1.log) · [0.8.0](ruffle-v0.8.0.log) · [Upstream 0.8.0 assets](https://github.com/ruffle-rs/ruffle/releases/expanded_assets/v0.8.0)
- The publisher's download server supplied both AionUi Apple Silicon DMGs. Their bundle was `com.aionui.app`, signed by AionUi Inc. (`52JQX2HUSC`), with a minimum of macOS 11.0. The GitHub tag alone does not supply these DMGs. The 2.2.1 digest was measured from the publisher's download. The 2.2.2 digest also matched the Homebrew cask receipt. [2.2.1](aionui-v2.2.1.log) · [2.2.2](aionui-v2.2.2.log) · [Publisher downloads](https://www.aionui.com/download/) · [Homebrew receipt](https://formulae.brew.sh/cask/aionui)

Copy `preflight-ruffle.sh` or `preflight.sh` into a fresh marked Air scratch directory. Run the Ruffle script with the tag, byte count and SHA-256 shown in its receipt. Run the DMG script with the package filename, URL and byte count from the AionUi receipt. For the latest AionUi DMG, supply the cask digest as the final argument.

### Ruffle launcher lifecycle on Apple Silicon

On 28 November 2026, the launcher’s native backend installed 0.7.1 from the official archive on the Air, fetched and applied 0.8.0, then moved the updated app to the test profile’s Trash. Both packages matched the published SHA-256 values and the copied bundles passed signature, notarization and version checks. A background `--version` probe received `SIGTRAP` in the offline sandbox. The successful cycle covered installation, update and removal. [Native log](ruffle-native.log)

Save the GitHub expanded asset pages as `ruffle-v0.7.1-assets.html` and `ruffle-v0.8.0-assets.html` alongside their preflight logs. With the compiled `native-cycle` in the marked scratch directory, run `bash run-native.sh ruffle ruffle-rs/ruffle Ruffle v0.7.1 v0.8.0` while 0.8.0 is the latest stable release.

### AionUi launcher lifecycle on Apple Silicon

On 28 November 2026, the Air on macOS 26.6.2 installed 2.2.1, detected and applied 2.2.2, and moved the updated bundle to the test profile's Trash. Both packages were downloaded from the publisher and matched their SHA-256 receipts. Synthetic user data survived the update and removal. The proof used a two-release history cache to select the older version and a fresh lookup for the stable update. It exercised the native backend through its public commands. Opening the app and Windows, Linux and Intel Mac packages are outside this proof.

The Mac recipe accepts the two reviewed DMGs and requires the publisher's Developer ID signature. A later GitHub tag can be discovered before its DMG is reviewed, in which case that version has no in-app installer. [Release notes](https://github.com/iOfficeAI/AionUi/releases) · [Publisher feed](https://static.aionui.com/releases/latest-arm64-mac.yml) · [Native log](aionui-native.log)

Copy the two preflight logs and the publisher feed (as `aionui-macos-feed.yml`) to the marked directory alongside the compiled `native-cycle` and `run-native.sh`. Run `bash run-native.sh aionui iOfficeAI/AionUi AionUi v2.2.1 v2.2.2` while the feed advertises 2.2.2.

### Czkawka 12.0.2 on Apple Silicon

Both GUI downloads matched their published SHA-256 values. The GTK build is a bare ARM64 executable. The Krokiet archive contains an ARM64 executable and backend launch scripts. Both executables use linker ad hoc signatures without a Developer ID Team and Gatekeeper rejected both on macOS 26.6.2. This was a package inspection without launching either binary. The catalog points to the upstream releases for Mac while the verified Flathub recipe stays available on Linux. [Publisher assets](https://github.com/qarmin/czkawka/releases/expanded_assets/12.0.2) · [Package log](czkawka-macos-package.log)

### Karabiner-Elements 17.0.0 on Apple Silicon

The DMG matched the publisher's SHA-256 and contained a single `.pkg`. Gatekeeper accepted the package as a Notarized Developer ID installer from Fumihiko Takayama (`G43BCU2T37`). It was inspected from a read-only mount. Installation, driver approval and service removal need a separate macOS QA system and remain outside this check. The catalog detail explains the initial macOS permissions. [Publisher assets](https://github.com/pqrs-org/Karabiner-Elements/releases/expanded_assets/v17.0.0) · [Setup instructions](https://karabiner-elements.pqrs.org/docs/getting-started/installation/) · [Package log](karabiner-macos-package.log)

### ScummVM launcher lifecycle on Apple Silicon

On 28 November 2026, the Air running macOS 26.6.2 installed 2026.3.0, discovered 2026.4.0, downloaded the official update and moved the updated bundle into the isolated profile's Trash. The native backend selected the old release from a two-release history cache and queried GitHub for the stable update. Both downloads matched the published SHA-256. Gatekeeper accepted the two universal bundles with ID `org.scummvm.app` and signing team `W5BM726PG6`. The ARM64 executable declares macOS 11.0 as its minimum and reported the expected version under an offline, write-denied sandbox after both installs. The synthetic user file survived. The proof covers Apple Silicon and command-line launch. Graphical launch and the Intel slice are outstanding. ScummVM is GPL-3.0-or-later and the launcher fetches the publisher's DMG directly. [Downloads](https://www.scummvm.org/downloads/) · [Native log](scummvm-native.log) · [2026.3.0 preflight](scummvm-2026.3.0.log) · [2026.4.0 preflight](scummvm-2026.4.0.log)

In a marked scratch directory, save the two preflight logs, the publisher's `scummvm-<version>-macosx.dmg.sha256` files and the official download page as `scummvm-downloads.html`. With the compiled `native-cycle` and `run-native.sh` in place, use `bash run-native.sh scummvm scummvm/scummvm ScummVM v2026.3.0 v2026.4.0` while 2026.4.0 is stable.

### Notepad Next and Pixelorama on Apple Silicon

The Notepad Next 0.15 ARM64 DMG and the Pixelorama 1.2.4 universal DMG matched the publisher-listed SHA-256 values. Strict verification rejected the Notepad Next bundle because the ad hoc signature expected resources that were missing. The Pixelorama bundle passed strict signature validation with an ad hoc signature and Gatekeeper rejected it. The DMGs were inspected on read-only mounts on November 28, 2026. The catalog keeps the publisher links for Mac. [Notepad Next package](https://github.com/dail8859/NotepadNext/releases/expanded_assets/v0.15) · [Notepad Next log](notepadnext-v0.15.log) · [Pixelorama package](https://github.com/Orama-Interactive/Pixelorama/releases/expanded_assets/v1.2.4) · [Pixelorama log](pixelorama-v1.2.4.log)

### WWDC launcher lifecycle on Apple Silicon

On 28 November 2026, the Air running macOS 26.6.2 installed WWDC 7.4.2, detected and applied 7.5, and moved the bundle into the isolated profile’s Trash. Both official DMGs contained the universal bundle `io.wwdc.app` and passed strict signature and Gatekeeper notarization checks for Guilherme Rambo (`8C7439RJLG`). The 7.5 digest also matched Homebrew’s cask metadata. The native launcher command started both versions and the harness checked the isolated executable path before closing each process. The synthetic user file survived. This run covers the Apple Silicon native backend. Graphical interaction, video playback and the Intel slice remain outside it. [Publisher releases](https://github.com/insidegui/WWDC/releases) · [Native cycle](wwdc-native.log) · [7.4.2 package](wwdc-7.4.2.log) · [7.5 package](wwdc-7.5.log)

Save the two official expanded asset pages as `wwdc-7.4.2-assets.html` and `wwdc-7.5-assets.html`, the Homebrew cask metadata as `wwdc-cask.json`, and the two preflight logs alongside the compiled probe. Run `bash run-native.sh wwdc insidegui/WWDC WWDC 7.4.2 7.5` while 7.5 is the latest release.

### QOwnNotes, NormCap and Caesium on Apple Silicon

On 28 November 2026 the Air verified the published SHA-256 of the current official DMGs and mounted them read-only. QOwnNotes 26.11.15 and 26.11.16 and NormCap 0.6.0 contain ARM64 executables. Strict signature checks passed, then the notarization requirement failed and the bundles reported no Developer ID team. Caesium 2.8.4 and 2.8.5 contain Intel-only executables. The old Caesium digest was measured from the official download. Package inspection ended before installation. Mac users keep the publisher links and the existing Linux recipes for QOwnNotes and NormCap are unaffected. [QOwnNotes log](qownnotes-macos-preflight.log) · [NormCap log](normcap-macos-preflight.log) · [Caesium log](caesium-macos-preflight.log)

### Pika launcher lifecycle on Apple Silicon

On 28 November 2026 the launcher installed Pika 2.0.2, discovered and applied 2.0.3, and moved the bundle to the private Trash on the Air running macOS 26.6.2. Both downloads matched the publisher’s SHA-256, passed Gatekeeper notarization, and contained the universal bundle `com.superhighfives.Pika`, signed by David Charles Gleason (`TGHU37N6EX`). The ARM64 slices require macOS 14.0. The native launch command started both versions, and the probe checked the exact executable path before terminating each isolated process. Synthetic user data survived. The color picker itself and the Intel slice remain outside this test. Pika is MIT licensed. [Publisher releases](https://github.com/superhighfives/pika/releases) · [Native cycle](pika-color-native.log) · [Previous package](pika-color-2.0.2.log) · [Current package](pika-color-2.0.3.log)

Save the official expanded asset pages as `pika-color-2.0.2-assets.html` and `pika-color-2.0.3-assets.html` and reproduce each matching log with `preflight.sh`. Run `bash run-native.sh pika-color superhighfives/pika Pika 2.0.2 2.0.3` in the marked directory with the compiled probe while 2.0.3 is current.

### Logseq DB beta launcher lifecycle on Apple Silicon

On 28 November 2026 the Air running macOS 26.6.2 installed the official Logseq DB 2.0.1 beta, opened it, detected and applied 2.0.2, opened the new version and moved the bundle into the isolated profile's Trash. Both ARM64 disk images matched the release digests and passed Developer ID and Gatekeeper notarization checks for Logseq Inc. (`K378MFWK59`). Both require macOS 12.0. The scoped launches used an explicit private user data directory, and synthetic data survived the cycle. This AGPL-3.0 recipe has its own catalog ID. Logseq DB detection rejects earlier file-edition versions and the installer protects an existing file-edition bundle even after a path-overwrite confirmation. The existing Linux file-edition installation route remains on its own ID. Migration, in-app editing and desktop packages for the other targets are outside this proof. [Publisher releases](https://github.com/logseq/logseq/releases) · [Edition split](https://discuss.logseq.com/t/whats-new-with-logseq-db-may-16th-2026/35020) · [Native cycle](logseq-db-native.log) · [2.0.1 package](logseq-db-2.0.1.log) · [2.0.2 package](logseq-db-2.0.2.log)

The read-only package inspection used the `logseq` scratch directory name before the edition was identified. Save the official expanded asset pages as `logseq-db-2.0.1-assets.html` and `logseq-db-2.0.2-assets.html` and reproduce the package checks with `preflight.sh`. With the logs and the compiled probe in the marked directory, run `bash run-native.sh logseq-db logseq/logseq Logseq 2.0.1 2.0.2` while 2.0.2 is current.

### qView on Apple Silicon

On 28 November 2026 the Air matched the publisher’s SHA-256 for qView 7.0 and 7.1. Both disk images contained an ARM64 slice and the bundles passed deep, strict signature verification. Neither bundle had a signing team and both failed the notarization requirement. The macOS card keeps the publisher link. The Linux lifecycle is [verified separately](../pro-371-linux-desktop-six/README.md). [Publisher releases](https://github.com/jurplel/qView/releases) · [7.0 package](qview-7.0.log) · [7.1 package](qview-7.1.log)

### RawTherapee on Apple Silicon

Both official Mac ZIP archives matched the publisher’s SHA-256. The unattended read-only attach of the inner disk image was canceled for both 5.12 and 5.13. DiskImages reported an embedded Software License Agreement for 5.13. The Homebrew listing uses the same 5.13 ZIP. The checks stopped before signature validation or installation and the Mac card keeps the publisher link. [5.12 download](https://www.rawtherapee.com/downloads/5.12/) · [5.13 download](https://www.rawtherapee.com/downloads/5.13/) · [5.12 log](rawtherapee-5.12.log) · [5.13 log](rawtherapee-5.13.log)
