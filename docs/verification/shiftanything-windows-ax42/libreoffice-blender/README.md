# LibreOffice and Blender Windows x64 package QA

Tested in the isolated Windows 11 Enterprise Evaluation VM on AX42 on November 28, 2026. These are operating system package tests performed with Windows Installer. This report covers the native package tests. A separate launcher candidate must pass its own tests before the launcher advertises either package as verified.

## Official packages

| App | Old MSI | New MSI | Timestamped Authenticode publisher |
| --- | --- | --- | --- |
| LibreOffice | 26.8.2 (MSI 26.8.2.2) | 26.8.3 (MSI 26.8.3.1) | The Document Foundation |
| Blender | 5.2.4 | 5.2.5 | Blender Foundation |

The four packages matched the exact file sizes and SHA-256 digests in the respective publisher manifests. All four passed Authenticode and the Windows SDK x64 `signtool verify /pa /all /tw` check and declared x64 in the Windows Installer package summary. See the [pinned receipts](receipts.csv), [preflight script](msi-preflight.ps1), and [preflight output](01-msi-preflight.png).

[LibreOffice 26.8.2 Metalink](https://download.documentfoundation.org/libreoffice/stable/26.8.2/win/x86_64/LibreOffice_26.8.2_Win_x86-64.msi.meta4) · [LibreOffice 26.8.3 Metalink](https://download.documentfoundation.org/libreoffice/stable/26.8.3/win/x86_64/LibreOffice_26.8.3_Win_x86-64.msi.meta4) · [Blender 5.2.4 digests](https://download.blender.org/release/Blender5.2/blender-5.2.4.sha256) · [Blender 5.2.5 digests](https://download.blender.org/release/Blender5.2/blender-5.2.5.sha256)

## LibreOffice

Installed the older official MSI in the guest's normal Windows Installer machine context. Windows registered `LibreOffice 26.8.2.2` and its executable at `C:\Program Files\LibreOffice\program\soffice.exe`. LibreOffice Writer opened in a dedicated standard user profile. Windows Installer then upgraded to `26.8.3.1` and replaced the older product registration. [Install](02-libreoffice-installed.png) · [Writer in the standard profile](03-libreoffice-writer.png) · [Upgrade](04-libreoffice-updated.png)

Writer reopened and showed version 26.8.3.1 (x86_64) in About. Windows reported both `soffice.exe` and `soffice.bin` under the standard QA user, whose synthetic profile marker remained in place. [Updated Writer](05-libreoffice-updated-writer.png) · [Process owner and profile marker](06-libreoffice-process-owner.png)

Windows Installer removed LibreOffice 26.8.3.1 and cleared its registration and executable. The standard user profile marker persisted. [Removal](07-libreoffice-removed.png)

A launcher recipe must account for the four-component MSI registry version, the version suffix in DisplayName, and the executable's nested `program` directory. LibreOffice's MSI uses `INSTALLLOCATION` for custom destinations. The existing launcher passes `INSTALLFOLDER` to its reviewed MSI packages. [LibreOffice executable documentation](https://help.libreoffice.org/latest/en-GB/text/shared/guide/start_parameters.html?System=WIN) · [The Document Foundation installer options](https://wiki.documentfoundation.org/images/9/9c/01ES_15LibreOfficeInstallation_V43.pdf)

## Blender

Windows Installer installed 5.2.4 and upgraded to 5.2.5, replacing the old product registration. Launching either version as the standard user produced Blender's error that the virtual graphics device lacks OpenGL 4.3. The guest could not open Blender's editor. Windows Installer removed 5.2.5, cleared the registration and executable, and preserved the standard user profile marker. [Graphics error](08-blender-graphics-check.png) · [Removal](09-blender-removed.png) · [Blender compatibility requirements](https://developer.blender.org/docs/release_notes/compatibility/)

## Reproduce in a disposable Windows x64 VM

1. Run [the preflight](msi-preflight.ps1) to download the four packages, check the pinned receipts, and write the local evidence. See the parameters in the script for the Windows SDK SignTool and evidence directory.
2. From an elevated Windows PowerShell in the VM, dot-source [the MSI cycle](msi-cycle.ps1) from the same folder as [the standard-user launch helper](launch-app.ps1). Run `Start-QA` once to create the temporary standard account.
3. For each app (`office` or `blender`), run `Invoke-QAMsi <app> install`, then `Open-QAApp <app>`. Close the app or graphics error before `Invoke-QAMsi <app> update`. Repeat the open check before `Invoke-QAMsi <app> remove`.
4. Run `Read-QAProfile` after removal. Run `Clear-QAFixture` after both products are removed to clean up the account, profile, and downloaded packages.
