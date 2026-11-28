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
