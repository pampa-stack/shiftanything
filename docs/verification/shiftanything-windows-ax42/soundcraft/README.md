# SoundCraft Windows x64 QA

Tested on November 28, 2026, in a disposable Windows 11 Enterprise Evaluation guest on AX42 with QEMU/KVM. The guest used a writable overlay backed by a separate read-only copy of the original Windows disk and its own TPM and flash state. The two official MSI fixtures entered the guest through a read-only ISO.

## Package checks

Both official MSI files matched their [0.20.0](https://github.com/storytold/soundcraft/releases/download/v0.20.0/SHA256SUMS.txt) and [0.20.1](https://github.com/storytold/soundcraft/releases/download/v0.20.1/SHA256SUMS.txt) checksums and passed Windows SDK x64 SignTool `/pa /all /tw`. Authenticode validated the timestamped Learning Machines Inc signature for both. The MSI metadata identifies SoundCraft and Learning Machines LLC, versions 0.20.0 and 0.20.1, template `x64;1033`, and `ALLUSERS=2`. [Package receipts](receipts.csv) · [Windows preflight](01-signatures.png)

The Windows x64 selector binds the approved repository, release version, asset URL, asset name, length and SHA-256. Its existing rejection suite now exercises both admitted SoundCraft versions. It rejects unknown versions, prereleases, x86, ARM64, ZIP and mismatched receipts.

## Windows x64 launcher

The isolated guest ran the direct Sh*ftAnything 0.20.9 Release executable for `x86_64-pc-windows-msvc` in its disposable administrator profile. The executable was built offline with `windows-lifecycle` from `beac6c35dc99b6fbbf89cca0a79ce008e5df2d3e` plus the four SoundCraft source changes in [the input hashes](launcher-source-sha256.csv). The resulting executable was 69,819,904 bytes with SHA-256 `2981baf02386e0d38698de10927b6acfc1928a9cdf6d7b26a08792442409f696`. The [build commands](launcher-build.cmd), retained with normalized line endings, passed 160 core tests (1 ignored), 585 native tests (4 ignored), and the targeted Release Tauri manifest test. [Build and binary receipt](02-release-build.png)

The catalog showed **Install**, the verified Windows badge and the 15.4 MB download. After approval, the backend installed the official 0.20.1 MSI. **Open** launched the DAW timeline and its About box reported version 0.20.1. **Remove** cleared the first installation. [Catalog](03-install.png) · [Windows Installer](04-installing.png) · [First opening](05-first-open.png) · [First removal](06-first-removal.png)

The signed 0.20.0 MSI seeded the update in machine scope. **Check for updates** detected 0.20.1. **Open** first launched 0.20.0 and its About box confirmed that version. After closing the editor, **Update** with approval showed the native installing progress, completed the upgrade and listed 0.20.1 with zero updates available. **Open** reopened the DAW and its About box confirmed 0.20.1. The independent Windows probe checked the new MSI registration, verified that the old one was absent, and located the executable in the machine installation folder. [Update available](07-update-available.png) · [Running 0.20.0](08-open-0.20.0.png) · [Update progress](09-update-progress.png) · [Update complete](10-updated.png) · [Running 0.20.1](11-reopened-0.20.1.png) · [Windows registration](12-updated-registry.png)

**Remove** with approval returned the launcher to zero installed apps. The Windows probe independently confirmed that both MSI registrations, the application executable and process were absent. [Launcher removal](13-final-removal.png) · [Windows removal check](14-removal-system-check.png)

For reproduction, put the exact MSI files from the receipts alongside [preflight.ps1](preflight.ps1) in a disposable Windows x64 guest with the x64 Windows SDK SignTool. Run it and [probe.ps1](probe.ps1) from a QA administrator shell with PowerShell `RemoteSigned` scoped to that process. After a clean removal, `probe.ps1 seed` installs the verified older MSI. Use the launcher to run the approved update, then run `probe.ps1 new`. After the final launcher removal, run `probe.ps1 empty`.

The manifest admits SoundCraft on Windows x64 only. The VM had no audio device, so QA covered DAW startup and the observed versions. Playback, recording, a standard-user launcher cycle and public launcher signing were outside this run.
