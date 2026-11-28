# TextureCraft Windows x64 QA

Tested on November 28, 2026, on Windows 11 Enterprise Evaluation in an isolated AX42 QEMU/KVM VM. The Windows test used a writable overlay backed by a read-only copy of the previously validated guest disk, with separate TPM and flash state. The two upstream packages entered the guest on a read-only ISO.

| Version | Bytes | Signed publisher | Windows Installer manufacturer |
| --- | ---: | --- | --- |
| 0.6.0 | 35,454,976 | Learning Machines Inc | Learning Machines LLC |
| 0.6.1 | 35,463,168 | Learning Machines Inc | Learning Machines LLC |

Both exact official MSI files matched their published [0.6.0 checksums](https://github.com/storytold/texturecraft/releases/download/v0.6.0/SHA256SUMS.txt) and [0.6.1 checksums](https://github.com/storytold/texturecraft/releases/download/v0.6.1/SHA256SUMS.txt). Both passed the Windows SDK x64 SignTool verification with `/pa /all /tw`. Windows reported Authenticode `Valid` and a timestamp, and the MSI metadata confirmed the exact versions, x64 architecture, manufacturer and machine scope. [Receipts](receipts.csv) · [Preflight output](01-msi-preflight.png)

Windows Installer installed 0.6.0, and the sample scene rendered in the real editor under a disposable standard user. Windows Installer upgraded in place to 0.6.1, removing the old application registration. The real editor reopened and reported version 0.6.1 with the Microsoft Basic Render Driver in its status bar. [Opened 0.6.0](02-0.6.0-open.png) · [Verified standard-user process](03-0.6.0-standard-owner.png) · [Opened 0.6.1](04-0.6.1-open.png) · [Updated MSI registration](05-0.6.1-registered.png)

Removal cleared both MSI product registrations and the executable. The synthetic standard-user profile marker survived the update and uninstall. [Removal and marker](06-removed.png)

To reproduce the native cycle, place the two MSI files specified in the receipts alongside the scripts in a disposable Windows x64 VM. Run [run-qa.ps1](run-qa.ps1) from an elevated PowerShell session with `RemoteSigned` scoped to that process. Start the temporary standard user with `Start-QA`. Run `Invoke-QAMsi texturecraft install`, `Open-QAApp texturecraft` and `Confirm-QAOwner texturecraft`. Close the app, then repeat for `update`. Close it again before `remove`. Check the profile marker with `Read-QAProfile` prior to cleanup. The scripts need the Windows SDK x64 SignTool.

## Launcher lifecycle

The same isolated Windows guest ran a disposable administrator profile and an internal Sh*ftAnything 0.20.9 Release executable for `x86_64-pc-windows-msvc`, built with `--features windows-lifecycle` from `bbfecd2` plus `f2e5293`, `58419e0`, and the TextureCraft manifest entry recorded in [source hashes](launcher-source-sha256.txt). The unsigned test executable was 69,819,904 bytes with SHA-256 `ed016d5db0e74f877c5046334d24daed986382730f6a3f5504ef44885f1204c6`. [Build and executable receipt](07-launcher-release-receipt.png)

The catalog showed **Install**, the verified Windows badge and a 35.5 MB download. Install with approval used the official 0.6.1 MSI through the native backend. The launcher detected 0.6.1, opened the real TextureCraft editor with approval, and removed the app via **My apps**. [Install](08-launcher-install.png) · [Windows Installer](09-launcher-installing.png) · [Opened with version 0.6.1](10-launcher-first-open.png) · [Removal](11-launcher-first-remove.png)

The [update seed](seed-launcher-update.ps1) independently installed the signed 0.6.0 MSI. **Check for updates** detected 0.6.1. **Update** and approval showed native download progress, completed the MSI upgrade and detected 0.6.1 with zero available updates. **Open** with approval reopened the real 0.6.1 editor. [Update available](12-launcher-update-available.png) · [Download progress](13-launcher-downloading.png) · [Updated registration](14-launcher-updated.png) · [Reopened editor](15-launcher-reopened-editor.png)

**Remove** with approval cleared the updated app from **My apps**. An [independent system check](check-launcher-removal.ps1) confirmed both MSI product registrations, the executable at its MSI location and the app process were absent. [Launcher result](16-launcher-final-remove.png) · [System check](17-launcher-removal-system-check.png)

Windows x64 tests before the Release build: core 160 passed and 1 ignored, native 585 passed and 4 ignored. This verifies a direct Release executable and official TextureCraft packages in the test guest. The scope of the manifest is TextureCraft on Windows x64. Launcher signing, an NSIS package of this candidate, and Windows ARM64 remain outside this proof.
