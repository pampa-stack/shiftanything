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

## Logseq file graphs

The native package cycle passed on 27 November 2026 for the [0.10.14](https://github.com/logseq/logseq/releases/tag/0.10.14) and [0.10.15](https://github.com/logseq/logseq/releases/tag/0.10.15) Apple Silicon DMGs. The native verifier installed 0.10.14, replaced it with 0.10.15 from the same signing team `3K44EUN829` and bundle ID `com.electron.logseq`, detected the new ARM64 bundle, and removed it into a temporary Trash. The synthetic profile marker survived. App launch and opening a graph were outside the test.

| Release | Official asset | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| 0.10.14 | `Logseq-darwin-arm64-0.10.14.dmg` | 187992621 | `9efbd89ff4bb7e4cca951d665be2c900d972cdc55a552570c88c6c866b2c1b9d` |
| 0.10.15 | `Logseq-darwin-arm64-0.10.15.dmg` | 199396040 | `a0b83e5bdc2b8bb639cec5747cedcc0f0a8cd840ca6fa0d44242dc05312d27cb` |

The catalog has no native Logseq recipe. The original repository's latest channel now serves the separate DB edition, which uses team `K378MFWK59` and bundle ID `com.logseq.logseq`. The [Logseq project documents the split](https://github.com/logseq/docs/blob/master/db-version-changes.md) and publishes newer file-graph packages from [Logseq OG](https://github.com/logseq/og/releases). A recovered 2.0.1-to-2.0.2 package test was for DB beta and does not validate the catalog's file-graph edition.

## OpenHands Agent Canvas

The [1.46.0](https://github.com/OpenHands/OpenHands/releases/tag/v1.46.0) official Universal DMG matched its published SHA-256 and contained a Universal Electron main executable. macOS reported no Developer ID team and rejected the bundle in deep, strict code-signature verification. The app stayed outside the verified Mac set.
