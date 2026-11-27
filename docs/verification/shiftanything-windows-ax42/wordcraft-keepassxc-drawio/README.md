# Windows x64 candidate packages

The packages were examined in the isolated Windows 11 Enterprise Evaluation x64 guest on AX42. The baseline was `1914684` (`v0.20.8`). The working build enables the candidates in a guest-only verification manifest.

| App | Old MSI | New MSI | Authenticode publisher | Windows Installer manufacturer |
| --- | --- | --- | --- | --- |
| WordCraft | [0.18.0](https://github.com/storytold/wordcraft/releases/expanded_assets/v0.18.0) | [0.19.1](https://github.com/storytold/wordcraft/releases/expanded_assets/v0.19.1) | Learning Machines Inc | Learning Machines LLC |
| KeePassXC | [2.7.12](https://github.com/keepassxreboot/keepassxc/releases/expanded_assets/2.7.12) | [2.8.0](https://github.com/keepassxreboot/keepassxc/releases/expanded_assets/2.8.0) | DroidMonkey Apps, LLC | KeePassXC Team |
| draw.io Desktop | [31.5.2](https://github.com/jgraph/drawio-desktop/releases/expanded_assets/v31.5.2) | [32.7.4](https://github.com/jgraph/drawio-desktop/releases/expanded_assets/v32.7.4) | draw.io Ltd | JGraph |

The six upstream MSI downloads matched the byte counts and published SHA-256 digests pinned in `crates/core/src/release.rs`. Windows returned Authenticode `Valid` with a timestamp for all six. The Windows Installer Summary Information Template declared x64 for each package. The MSI ProductName for draw.io Desktop is `draw.io`. [Authenticode results](01-authenticode.png) · [MSI properties](02-msi-identity.png)

These are package checks. In-launcher installation, opening, updates and removal need separate evidence for each candidate.

The Windows SDK 10.0.26100.0 x64 SignTool verified the WordCraft 0.19.1 MSI under the default Authenticode policy and found an RFC 3161 timestamp. The release verifier rejected the same signed MSI with an unrelated expected publisher and rejected the known unsigned Sh*ftAnything 0.20.6 NSIS installer. The check used `signtool verify /pa /all /tw <artifact>` and `Get-AuthenticodeSignature -LiteralPath <artifact>`. [Signed, wrong-publisher and unsigned controls](03-signtool-controls.png) · [Microsoft SignTool documentation](https://learn.microsoft.com/windows/win32/seccrypto/signtool) · [Microsoft PowerShell documentation](https://learn.microsoft.com/powershell/module/microsoft.powershell.security/get-authenticodesignature)
