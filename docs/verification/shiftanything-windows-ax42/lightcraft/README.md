# LightCraft Windows x64 QA

Tested on November 28, 2026, in a disposable Windows 11 Enterprise Evaluation guest on AX42 with QEMU/KVM. The writable test overlay used a read-only copy of the existing Windows fixture disk and separate TPM and UEFI state. The official MSI and exact candidate sources entered the guest on read-only media.

The 0.21.0 and 0.21.1 MSI matched the publisher's [0.21.0 SHA256SUMS](https://github.com/storytold/lightcraft/releases/download/v0.21.0/SHA256SUMS.txt) and [0.21.1 SHA256SUMS](https://github.com/storytold/lightcraft/releases/download/v0.21.1/SHA256SUMS.txt). Both passed Windows SDK x64 SignTool `/pa /all /tw`, with valid Authenticode signature and timestamp, Learning Machines Inc signing publisher, Learning Machines LLC MSI manufacturer, x64 and exact ProductVersion. [Receipts](receipts.csv) · [Preflight](01-signatures.png)

The recipe admits the exact two MSI assets from `storytold/lightcraft` on Windows x64. Regression coverage exercises mismatched SHA-256, length, repository and version, plus prerelease, ZIP, x86 and ARM64 rejection.

## Launcher lifecycle


The local Release executable is Sh*ftAnything 0.20.9, x86_64-pc-windows-msvc with `windows-lifecycle`. It was built from `8777fdad2a2a4697d1fdcf56b58eca8d98f4ebb6` with the four source files in this change applied. The executable was 71,682,560 bytes, SHA-256 `87709be40000a1963a533e619961972b0c601143fbb27e4e6ec7468f8f01b4c7`. [Build and binary receipt](02-release-build.png)

1. Installed 0.21.1 from the clean catalog with approval. Windows Installer configured the app. Open from Sh*ftAnything rendered the bundled demo library. The optional lens profile download was declined. Remove returned the UI to zero installed apps. [Approval](03-install-consent.png) · [Native install](04-native-install.png) · [Library](05-first-library.png) · [First removal](06-first-removal.png)
2. Installed the official 0.21.0 MSI to seed a previous version. The independent probe checked the MSI registration and installed executable. Sh*ftAnything detected 0.21.0 with an available update. Open launched the old window, which independently showed its 0.21.0 to 0.21.1 notice. Closed the app without using its updater. [Older MSI](07-previous-msi.png) · [Older GUI](08-previous-open.png)
3. Approved Update in My apps and observed Windows Installer and launcher progress. My apps showed 0.21.1 and no outstanding update. Open launched the new window. The independent probe then found exactly the newer MSI registration and the executable. [Approval](09-update-consent.png) · [Native update](10-native-update.png) · [Installed version](11-updated.png) · [Reopened](12-reopened.png) · [Newer MSI](13-updated-msi.png)
4. Closed LightCraft and approved Remove in My apps. It returned to zero installed apps. The independent probe found neither ProductCode, executable nor running process. [Empty My apps](14-my-apps-empty.png) · [Removal probe](15-removal-probe.png)

The test used an administrator in the disposable guest and LightCraft's bundled demo images. Standard-user UAC, hardware-accelerated performance, other CPU architectures and the public distribution of the launcher were outside this test.

## Reproduction

[Package preflight](preflight.ps1) and [independent lifecycle probes](probe.ps1) are included. Place both official MSI files and the receipt CSV together, then run `preflight.ps1` in an isolated Windows x64 test guest with the Windows SDK installed. `probe.ps1 seed` installs the older fixture after checking its receipt again, `probe.ps1 new` checks the newer version and `probe.ps1 empty` verifies final removal after the UI steps.

The [build script](launcher-build.ps1), [Cargo commands](launcher-build.cmd) and [transport SHA-256 receipt](launcher-source-sha256.csv) record the staged build. The transport filenames were `build.ps1`, `build.cmd` and `source-sha.csv`. The script copies here use LF, while the original transport used CRLF. Run the scripts against a clean guest build fixture with the Rust MSVC target, C++ Build Tools, Windows SDK and cached dependencies in place.

Windows Release verification passed 171 core tests (one ignored), 591 native tests (four ignored), the targeted Tauri admission test and the launcher build. The Windows package selection regression ran in the core suite. [Core regression output](16-core-test.png)
