# Easydict and UTM on macOS Apple Silicon

Tested November 28, 2026 on a MacBook Air running macOS 26.6.2 and `arm64`, with Gatekeeper enabled. The native proof used a fresh temporary home, Applications directory, cache and Trash for each run. The previous and current packages came directly from the publishers’ GitHub releases, with hashes checked against the official asset indexes. All four bundles include native `arm64` slices and passed strict signature verification and Gatekeeper assessment as notarized Developer ID applications. See [`preflight.log`](preflight.log).

| App | Previous → stable | Bundle, Developer ID team | Published package size (bytes) |
| --- | --- | --- | ---: |
| Easydict | [2.25.0](https://github.com/tisfeng/Easydict/releases/tag/2.25.0) → [2.26.0](https://github.com/tisfeng/Easydict/releases/tag/2.26.0) | `com.izual.Easydict`, `45Z6V4YD5U` | 35,766,921 → 35,761,660 |
| UTM | [5.0.8, build 128](https://github.com/utmapp/UTM/releases/tag/v5.0.8) → [5.0.9, build 132](https://github.com/utmapp/UTM/releases/tag/v5.0.9-2) | `com.utmapp.UTM`, `WDNLXAD4W8` | 304,304,581 → 304,699,821 |

The launcher reads the stable release through `NativeBackend`, downloads the real DMG and checks the byte count. The probe verifies the installed version and receipt after install and update, then checks removal to the temporary Trash and preservation of a synthetic user file. A first UTM attempt ran a helper probe in parallel with the update. The backend correctly stopped because there was a process in the bundle. The recorded UTM run repeats the cycle without the parallel probe. GUI launch was skipped for both apps because the Air SSH session is `Background` and has no isolated Aqua session. Easydict can use system-wide shortcuts and request macOS permissions. No user app or data was touched. See [`easydict-native.log`](easydict-native.log) and [`utm-native.log`](utm-native.log).

The core tests passed with 161 passed and one previously ignored. The asset selection test covers wrong architecture, operating system, prerelease, URL, size and checksum. The UTM tag test checks that the stable version alias applies to the exact official repository and tag.

The reviewed Easydict source has a [GPL-3.0 license](https://github.com/tisfeng/Easydict/blob/2.26.0/LICENSE). Its README describes use for learning and communication, and the recipe downloads the original package directly from the author. UTM has an [Apache-2.0 license](https://github.com/utmapp/UTM/blob/v5.0.9-2/LICENSE) and a free download from [the publisher](https://getutm.app/). The only UTM tag alias is the official `v5.0.9-2` for the stable bundle version `5.0.9`. Filee and Xournal++ failed the signature or notarization checks and were left out of the macOS installation matrix.
