# PhotoCraft on Windows x64

On 2026-11-27, the native engine installed, launched, updated, relaunched and removed PhotoCraft inside the isolated Windows 11 x64 guest. The harness used an elevated administrator account and the native Rust install, detect, launch and uninstall functions from `b9ebab7`. The Tauri interface and standard-account elevation path were outside this cycle.

| Release and package | Size | SHA-256 |
| --- | ---: | --- |
| [0.23.0](https://github.com/storytold/photocraft/releases/tag/v0.23.0) `photocraft-0.23.0-windows-x64.msi` | 118,390,784 bytes | `eac24e0e5a675c3cb50d25752138d63c405f0fb9482163968f0553d04653c789` |
| [0.23.1](https://github.com/storytold/photocraft/releases/tag/v0.23.1) `photocraft-0.23.1-windows-x64.msi` | 118,427,648 bytes | `de757802d8a12a41718ba24248c281d83f3bbecd64c5230f46f10c7f62070671` |

The staging check downloaded both official MSIs over HTTPS, matched release `SHA256SUMS.txt` and observed Valid Authenticode signatures from Learning Machines Inc. The Rust engine then verified the cached bytes during each job. The Install and Update steps each detected one registration and the expected version at the vendor's default path, `C:\Program Files\ArtCraft\PhotoCraft\photocraft.exe`. The actual PhotoCraft window rendered via the application's CPU fallback. After removal, neither the MSI registration nor the executable remained. A synthetic user data file survived update and removal.

Evidence from the guest is available for [signatures](signatures.png), [installation](install.png), [opening PhotoCraft](open.png), [updating](update.png), [opening 0.23.1](open-updated.png) and [removal](remove.png).
