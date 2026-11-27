# LibreOffice and Blender Windows x64 package QA

Tested in the isolated Windows 11 Enterprise Evaluation VM on AX42 on November 28, 2026. The report includes Windows Installer package tests for both apps and an independent Sh*ftAnything lifecycle test for LibreOffice.

## Official packages

| App | Old MSI | New MSI | Timestamped Authenticode publisher |
| --- | --- | --- | --- |
| LibreOffice | 26.8.2 (MSI 26.8.2.2) | 26.8.3 (MSI 26.8.3.1) | The Document Foundation |
| Blender | 5.2.4 | 5.2.5 | Blender Foundation |

The four packages matched the exact file sizes and SHA-256 digests in the respective publisher manifests. All four also matched their exact MSI product versions and passed Authenticode and the Windows SDK x64 `signtool verify /pa /all /tw` check and declared x64 in the Windows Installer package summary. See the [pinned receipts](receipts.csv), [preflight script](msi-preflight.ps1), and [preflight output](01-msi-preflight.png).

[LibreOffice 26.8.2 Metalink](https://download.documentfoundation.org/libreoffice/stable/26.8.2/win/x86_64/LibreOffice_26.8.2_Win_x86-64.msi.meta4) · [LibreOffice 26.8.3 Metalink](https://download.documentfoundation.org/libreoffice/stable/26.8.3/win/x86_64/LibreOffice_26.8.3_Win_x86-64.msi.meta4) · [Blender 5.2.4 digests](https://download.blender.org/release/Blender5.2/blender-5.2.4.sha256) · [Blender 5.2.5 digests](https://download.blender.org/release/Blender5.2/blender-5.2.5.sha256)

## LibreOffice

Installed the older official MSI in the guest's normal Windows Installer machine context. Windows registered `LibreOffice 26.8.2.2` and its executable at `C:\Program Files\LibreOffice\program\soffice.exe`. LibreOffice Writer opened in a dedicated standard user profile. Windows Installer then upgraded to `26.8.3.1` and replaced the older product registration. [Install](02-libreoffice-installed.png) · [Writer in the standard profile](03-libreoffice-writer.png) · [Upgrade](04-libreoffice-updated.png)

Writer reopened and showed version 26.8.3.1 (x86_64) in About. Windows reported both `soffice.exe` and `soffice.bin` under the standard QA user, whose synthetic profile marker remained in place. [Updated Writer](05-libreoffice-updated-writer.png) · [Process owner and profile marker](06-libreoffice-process-owner.png)

Windows Installer removed LibreOffice 26.8.3.1 and cleared its registration and executable. The standard user profile marker persisted. [Removal](07-libreoffice-removed.png)

The LibreOffice launcher recipe maps the four-component MSI registry version to its public release, checks the version in DisplayName and finds the executable in the nested `program` directory. It passes `INSTALLLOCATION` to preserve the install root on updates. [LibreOffice executable documentation](https://help.libreoffice.org/latest/en-GB/text/shared/guide/start_parameters.html?System=WIN) · [The Document Foundation installer options](https://wiki.documentfoundation.org/images/9/9c/01ES_15LibreOfficeInstallation_V43.pdf)

## Blender

Windows Installer installed 5.2.4 and upgraded to 5.2.5, replacing the old product registration. Launching either version as the standard user produced Blender's error that the virtual graphics device lacks OpenGL 4.3. The guest could not open Blender's editor. Windows Installer removed 5.2.5, cleared the registration and executable, and preserved the standard user profile marker. [Graphics error](08-blender-graphics-check.png) · [Removal](09-blender-removed.png) · [Blender compatibility requirements](https://developer.blender.org/docs/release_notes/compatibility/)

## Reproduce in a disposable Windows x64 VM

1. Run [the preflight](msi-preflight.ps1) to download the four packages, check the pinned receipts, and write the local evidence. The script looks for the Windows SDK x64 SignTool and writes evidence under `C:\sq\win-office-qa`.
2. From an elevated Windows PowerShell in the VM, dot-source [the MSI cycle](msi-cycle.ps1) from the same folder as [the standard-user launch helper](launch-app.ps1). Run `Start-QA` once to create the temporary standard account.
3. For each app (`office` or `blender`), run `Invoke-QAMsi <app> install`, then `Open-QAApp <app>`. Close the app or graphics error before `Invoke-QAMsi <app> update`. Repeat the open check before `Invoke-QAMsi <app> remove`.
4. Run `Read-QAProfile` after removal. Run `Clear-QAFixture` after both products are removed to clean up the account, profile, and downloaded packages.

## LibreOffice through Sh*ftAnything

The Windows x64 Release candidate uses product version 0.20.8, source baseline `1914684` with the four existing Windows recipes, the Refresh fix from `88adc0d`, and the exact LibreOffice changes in [the candidate source receipts](launcher-source-receipts.csv). It was built with `windows-lifecycle` and run as a temporary standard user. Windows Installer elevation used Windows' regular UAC prompts. The downloaded vendor MSI was signed. The launcher test binary was a local build.

From the catalog, Install showed the reviewed 376.3 MB Windows package, asked for consent, and displayed actual download progress. The Windows prompt identified The Document Foundation and LibreOffice 26.8.3.1. Upon completion, the launcher detected public version 26.8.3. Open displayed the real LibreOffice Start Center. The Remove action in My apps prompted for consent, and normal Windows Installer removal left the list empty. [Install](10-launcher-install.png) · [Download](11-launcher-download.png) · [Vendor UAC](12-launcher-signed-uac.png) · [Start Center](13-launcher-open.png) · [Removed](14-launcher-fresh-removed.png)

For the update test, the reviewed 26.8.2 MSI was seeded through Windows Installer. Refresh detected 26.8.2 and offered 26.8.3. Update requested consent and downloaded the 376.3 MB official package. After standard UAC, the launcher reported Action completed with zero pending updates. Open started the new version in the same standard user profile. Windows recorded DisplayVersion 26.8.3.1 and the expected executable, with the older product registration absent. The synthetic profile marker persisted. [Pending update](15-launcher-update-found.png) · [Download](16-launcher-update-download.png) · [Updated](17-launcher-updated.png) · [Reopened](18-launcher-reopened.png) · [Registry and process owner](19-launcher-upgrade-registry.png)

The x64 Release build passed. Rust tests passed for core (155 passed, one ignored) and native (573 passed, four ignored). The local executable was 64,757,248 bytes, SHA-256 `EB54133599DEBE30B01FA1270A6CA6A1A09779392E48D674976FD3EF9C600533`. Strict Windows Clippy reported existing warnings in other paths. [Build and tests](20-launcher-build-tests.png)

The TDF release adapter resolves 26.8.2 and 26.8.3 from the reviewed Windows x64 receipts. The catalog's release date remained unavailable during this proof. Each new Windows release or architecture needs its own reviewed receipt.
