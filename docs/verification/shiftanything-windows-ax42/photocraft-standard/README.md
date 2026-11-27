# PhotoCraft via the Windows launcher

Verified on 27 November 2026 in an isolated Windows 11 Enterprise Evaluation x64 virtual machine on AX42. The Tauri Release build was based on `d2a7abe` with the functional changes in `2e2df4a`. The build used `--features windows-lifecycle` and a guest-only copy of the verification manifest admitting just PhotoCraft for Windows x64. The repository manifest ships without that test entry.

## Packages

| Version | Upstream asset | Bytes | Published and verified SHA256 |
| --- | --- | ---: | --- |
| 0.23.0 | [Windows x64 MSI and checksums](https://github.com/storytold/photocraft/releases/tag/v0.23.0) | 118390784 | `eac24e0e5a675c3cb50d25752138d63c405f0fb9482163968f0553d04653c789` |
| 0.23.1 | [Windows x64 MSI and checksums](https://github.com/storytold/photocraft/releases/tag/v0.23.1) | 118427648 | `de757802d8a12a41718ba24248c281d83f3bbecd64c5230f46f10c7f62070671` |

Both packages had valid timestamped Authenticode signatures from Learning Machines Inc in the prior native package cycle. The launcher checks the pinned bytes and Windows Installer runs its usual UAC check. Version, source URL, asset name, size, and SHA256 must match the reviewed Windows x64 receipt. An unreviewed version, ZIP, x86, or Windows ARM64 asset is ineligible.

## Standard-user cycle

1. [Verify the fixture account is a standard user](01-standard-profile.png) and launch Sh*ftAnything under that identity.
2. Select Install and Reject the launcher confirmation once. [Install remains available](02-install-rejected.png). Select Install again, approve the launcher confirmation and provide an authorized local QA administrator at the real Windows UAC prompt. [My apps reports PhotoCraft 0.23.1](03-installed-0.23.1.png).
3. Select Open and approve. [PhotoCraft opens in its own foreground window](04-app-open.png). Close it and remove it from My apps, approving both the launcher and Windows prompts. [My apps reports zero installed applications](09-removed.png).
4. Seed the official 0.23.0 MSI in the guest with `ALLUSERS=1` after checking its SHA256 and Authenticode signature. Place a synthetic marker under the standard user's PhotoCraft roaming data. Close and restart the launcher as that same standard user.
5. [My apps shows 0.23.0 installed, 0.23.1 available and the orange Update button](05-update-available.png). Approve Update. [The actual download showed 51% with a progress bar](06-download-progress.png). [Windows UAC identified the verified publisher and requested an administrator](07-windows-consent.png). [My apps confirmed 0.23.1, action complete, zero updates, and a blue Open button](08-updated-0.23.1.png).
6. Remove 0.23.1 again using the launcher and Windows prompts. [The machine-wide executable was absent and the synthetic profile marker survived](10-binary-removed-marker-preserved.png).

## Observations and scope

- While the launcher was open, Check for updates did not discover the newly seeded external installation. Restarting detected it. The test of in-launcher updates continued from that detected state.
- Before the initial Rust snapshot, the Windows grid briefly displayed macOS download metadata with its action placeholders. It switched to Windows metadata after initialization.
- Machine-wide MSI install, update, and removal need a Windows administrator's approval when initiated by a standard user. The launcher itself ran without elevation. The separate elevated setup commands here produced only the checked old-version fixture and collected evidence.
- This proof applies to the two listed PhotoCraft MSIs on Windows x64 and the tested launcher build. Launcher self-update, bulk update, other Windows apps, and public Windows release signing were outside this proof.
