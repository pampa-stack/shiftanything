# Windows x64 candidate packages

The packages were examined in the isolated Windows 11 Enterprise Evaluation x64 guest on AX42. The baseline was `1914684` (`v0.20.8`). The working build enables the candidates in a guest-only verification manifest.

| App | Old MSI | New MSI | Authenticode publisher | Windows Installer manufacturer |
| --- | --- | --- | --- | --- |
| WordCraft | [0.18.0](https://github.com/storytold/wordcraft/releases/expanded_assets/v0.18.0) | [0.19.1](https://github.com/storytold/wordcraft/releases/expanded_assets/v0.19.1) | Learning Machines Inc | Learning Machines LLC |
| KeePassXC | [2.7.12](https://github.com/keepassxreboot/keepassxc/releases/expanded_assets/2.7.12) | [2.8.0](https://github.com/keepassxreboot/keepassxc/releases/expanded_assets/2.8.0) | DroidMonkey Apps, LLC | KeePassXC Team |
| draw.io Desktop | [31.5.2](https://github.com/jgraph/drawio-desktop/releases/expanded_assets/v31.5.2) | [32.7.4](https://github.com/jgraph/drawio-desktop/releases/expanded_assets/v32.7.4) | draw.io Ltd | JGraph |

The six upstream MSI downloads matched the byte counts and published SHA-256 digests pinned in `crates/core/src/release.rs`. Windows returned Authenticode `Valid` with a timestamp for all six. The Windows Installer Summary Information Template declared x64 for each package. The MSI ProductName for draw.io Desktop is `draw.io`. [Authenticode results](01-authenticode.png) · [MSI properties](02-msi-identity.png)

KeePassXC and draw.io Desktop remain package checks only. Their in-launcher lifecycle needs separate evidence.

The Windows SDK 10.0.26100.0 x64 SignTool verified the WordCraft 0.19.1 MSI under the default Authenticode policy and found an RFC 3161 timestamp. The release verifier rejected the same signed MSI with an unrelated expected publisher and rejected the known unsigned Sh*ftAnything 0.20.6 NSIS installer. The check used `signtool verify /pa /all /tw <artifact>` and `Get-AuthenticodeSignature -LiteralPath <artifact>`. [Signed, wrong-publisher and unsigned controls](03-signtool-controls.png) · [Microsoft SignTool documentation](https://learn.microsoft.com/windows/win32/seccrypto/signtool) · [Microsoft PowerShell documentation](https://learn.microsoft.com/powershell/module/microsoft.powershell.security/get-authenticodesignature)

## WordCraft in the launcher

Built Sh*ftAnything 0.20.8 for `x86_64-pc-windows-msvc` in Release with `windows-lifecycle`. The guest build used source tree `f14af309b9fbd0ca28b9f5fc74bc91fc39c598e0` and a guest-only candidate manifest. The unsigned QA executable had SHA-256 `ef8a9d7c75e3afbe0328ab21305c487945aad364cd0ca64eefab46dbdaa184e9`. Launcher and app windows ran as the temporary standard guest user. Windows Installer elevation used the guest administrator through the normal UAC prompt. [Standard user identity](04-standard-user.png)

1. From an empty inventory, **Install** downloaded the official 103,383,040-byte WordCraft 0.19.1 MSI and displayed progress. After approval and UAC, **Open** reached the actual WordCraft home screen with its taskbar icon. **Remove** returned My apps to zero. [Download](05-wordcraft-download.png) · [WordCraft home](06-wordcraft-open.png) · [Removed from My apps](07-wordcraft-remove.png)
2. Seeded the official 0.18.0 MSI under Windows Installer once and created a synthetic roaming-profile marker. The existing launcher window did not find the external install after **Check for updates**. Relaunching under the same standard user detected 0.18.0 and offered 0.19.1. [Refresh gap](10-inventory-refresh-gap.png)
3. **Update** while the WordCraft window was running was refused and kept 0.18.0 installed. After a normal close, the same action downloaded the official MSI and upgraded to 0.19.1. The registry held one matching MSI entry and the installed executable had a valid timestamped publisher signature. [App in use](08-wordcraft-running-block.png) · [Update download](09-wordcraft-update-download.png) · [Updated inventory](11-wordcraft-updated.png) · [Registry and executable](12-wordcraft-installed-signature.png)
4. **Remove** after the update cleared the MSI entry and executable while leaving the marker intact. [Cleanup and data preserved](13-wordcraft-clean.png)

The candidate branch enables WordCraft alongside PhotoCraft for Windows x64. The pin remains limited to the two reviewed MSI versions. The launcher binary above was a local QA build.
