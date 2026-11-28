$ErrorActionPreference = 'Stop'
$source = Join-Path $PSScriptRoot 'texturecraft-0.6.0-windows-x64.msi'
if ((Get-FileHash -LiteralPath $source).Hash -ne '9da7b303ac4027c2972aad655b9c89b294bb70c23758962f3a31310c8e8339d0') { throw 'Old MSI SHA256 mismatch' }
if ((Get-AuthenticodeSignature -FilePath $source).Status -ne 'Valid') { throw 'Old MSI Authenticode invalid' }
$keys = @('{8F0BDEC4-B7FF-4CF8-B924-57EF08E14A93}', '{7888A8AE-9EF7-453B-9EBF-93332A0C0B41}')
$reg = 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall'
foreach ($key in $keys) {
    if (Test-Path (Join-Path $reg $key)) { throw 'TextureCraft already registered' }
}
$proc = Start-Process msiexec.exe -ArgumentList @('/i', $source, 'ALLUSERS=1', '/qn', '/norestart') -Wait -PassThru
if ($proc.ExitCode -ne 0) { throw "Seed install exit $($proc.ExitCode)" }
if ((Get-ItemProperty -LiteralPath (Join-Path $reg $keys[0])).DisplayVersion -ne '0.6.0') { throw 'Expected old version' }
Write-Host 'QA_UI_UPDATE_SEED_PASS TextureCraft 0.6.0 exact official x64 MSI'
