# macOS Apple Silicon package verification

## DeepTutor

On 27 November 2026, the native Rust downloader and macOS DMG installer tested the following official Apache-2.0 desktop releases on an Apple Silicon Mac. The [1.7.3 release](https://github.com/HKUDS/DeepTutor/releases/tag/v1.7.3) and the [1.7.4 release](https://github.com/HKUDS/DeepTutor/releases/tag/v1.7.4) both publish a dedicated Apple Silicon DMG.

| Release | Official asset | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| 1.7.3 | `Open-DeepTutor-MacOS-Apple-Silicon-v1.7.3.dmg` | 504015269 | `cc48aa2dd80a78856233cfbf23d8934f2aebea9d1e69012a13923d59c5030699` |
| 1.7.4 | `Open-DeepTutor-MacOS-Apple-Silicon-v1.7.4.dmg` | 505892868 | `afe5bb50e9b29af77dbf1a03ab1e04e4db6112dcacb61d11ef7f822b9321dd79` |

- Both official HTTPS transfers, lengths, and digests matched. The native verifier accepted the Developer ID signature, team `ZP59KG8ABY`, bundle `org.deeptutor.open-desktop`, and target version before placing each copy.
- The package installed 1.7.3 and updated it to 1.7.4 in a temporary Applications directory. Both builds reported macOS 13 as the minimum and contained an ARM64 main executable.
- The native removal moved version 1.7.4 into a temporary Trash. The synthetic user-data marker survived update and removal. The test profile and downloaded DMGs were deleted.
- Evidence covers the package lifecycle. App launch, onboarding, model execution, and the launcher window were outside this test.

The recipe accepts the two reviewed Apple Silicon DMGs and their exact URLs, sizes, and digests. The API and quota-free release paths select the same file. macOS Intel, Linux, and future unreviewed assets get no install action from this recipe.

## OpenWork public desktop

The native installer passed the Apple Silicon package cycle on 27 November 2026 for the MIT-licensed public desktop edition from [0.18.72](https://github.com/different-ai/openwork/releases/tag/v0.18.72) and [0.18.73](https://github.com/different-ai/openwork/releases/tag/v0.18.73).

| Release | Official asset | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| 0.18.72 | `openwork-mac-arm64-0.18.72.dmg` | 382734040 | `13f1b2e4597df2d7938848629bbe9325a5c9ce6bc9399d4144048fa942e15af5` |
| 0.18.73 | `openwork-mac-arm64-0.18.73.dmg` | 383492503 | `6462fbe564667f37eeeabb7a7d5fc49c03d54f862364efc7f1dffa6473c5ec7b` |

- The native downloader verified official HTTPS, content length, and SHA-256. The native installer checked Developer ID team `F5DJWB4CCV`, bundle `com.differentai.openwork`, version, and notarization before installing 0.18.72 and replacing it with 0.18.73.
- Both builds reported macOS 12.0 as their minimum. Removal moved the updated ARM64 bundle into a temporary Trash. The synthetic profile marker survived update and removal. The isolated profile and both DMGs were deleted after the test.
- The recipe only accepts these two public DMGs. OpenWork's [public](https://github.com/different-ai/openwork/blob/v0.18.73/apps/desktop/electron-builder.yml), [Cloud](https://github.com/different-ai/openwork/blob/v0.18.73/apps/desktop/electron-builder.cloud.yml), and [Enterprise](https://github.com/different-ai/openwork/blob/v0.18.73/apps/desktop/electron-builder.enterprise.yml) configs share the bundle identifier and set different product names. Detection requires the public executable name, even if a Cloud or Enterprise bundle has been renamed to `OpenWork.app`.
- An uncached Gatekeeper assessment exceeded the standard 60-second limit. The repeat test used the exact catalog recipe and passed both package assessments with Gatekeeper enabled and the existing 180-second bound for large bundles.
- The test did not exercise GUI launch, agent connections, managed editions, or the launcher window.

## Logseq file graphs

The native package cycle passed on 27 November 2026 for the [0.10.14](https://github.com/logseq/logseq/releases/tag/0.10.14) and [0.10.15](https://github.com/logseq/logseq/releases/tag/0.10.15) Apple Silicon DMGs. The native verifier installed 0.10.14, replaced it with 0.10.15 from the same signing team `3K44EUN829` and bundle ID `com.electron.logseq`, detected the new ARM64 bundle, and removed it into a temporary Trash. The synthetic profile marker survived. App launch and opening a graph were outside the test.

| Release | Official asset | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| 0.10.14 | `Logseq-darwin-arm64-0.10.14.dmg` | 187992621 | `9efbd89ff4bb7e4cca951d665be2c900d972cdc55a552570c88c6c866b2c1b9d` |
| 0.10.15 | `Logseq-darwin-arm64-0.10.15.dmg` | 199396040 | `a0b83e5bdc2b8bb639cec5747cedcc0f0a8cd840ca6fa0d44242dc05312d27cb` |

The catalog has no native Logseq recipe. The original repository's latest channel now serves the separate DB edition, which uses team `K378MFWK59` and bundle ID `com.logseq.logseq`. The [Logseq project documents the split](https://github.com/logseq/docs/blob/master/db-version-changes.md) and publishes newer file-graph packages from [Logseq OG](https://github.com/logseq/og/releases). A recovered 2.0.1-to-2.0.2 package test was for DB beta and does not validate the catalog's file-graph edition.

## OpenHands Agent Canvas

The [1.46.0](https://github.com/OpenHands/OpenHands/releases/tag/v1.46.0) official Universal DMG matched its published SHA-256 and contained a Universal Electron main executable. macOS reported no Developer ID team and rejected the bundle in deep, strict code-signature verification. The app stayed outside the verified Mac set.

## Paseo desktop

The Apple Silicon package cycle passed on 27 November 2026 for the Apache-2.0 desktop edition. The native installer downloaded the official DMGs, verified their size and SHA-256, checked Developer ID team `99ZMJMKU9Y`, bundle `sh.paseo.desktop` and notarization, installed 0.17.6, replaced it with 0.18.0, and moved the updated ARM64 bundle into a temporary Trash. Both bundles declare macOS 13.0 as the minimum. The synthetic profile marker survived, and the temporary profile and downloads were removed. [0.17.6 release](https://github.com/getpaseo/paseo/releases/tag/v0.17.6) · [0.18.0 release](https://github.com/getpaseo/paseo/releases/tag/v0.18.0)

| Release | Official asset | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| 0.17.6 | `Paseo-0.17.6-arm64.dmg` | 190151170 | `3807556e33e6680a982138c534033b1e2101a6fc5a1c84ceb53d3614be6dfce5` |
| 0.18.0 | `Paseo-0.18.0-arm64.dmg` | 190247198 | `2fa0fe84825e3e4df60d6d8625adcd23995868f85a378206fe0e4cf8a69c1f91` |

The initial 0.18.0 assessment hit the 60-second Gatekeeper limit. The repeat passed with Gatekeeper enabled and the existing 180-second bound for large bundles applied to Paseo. Both release lookup paths use the exact reviewed macOS assets. This test covers the package lifecycle. The desktop window and agent connections were outside the test.
