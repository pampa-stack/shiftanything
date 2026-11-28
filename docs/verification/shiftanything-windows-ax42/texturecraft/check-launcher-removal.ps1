$ErrorActionPreference = 'Stop'
$keys = @('{8F0BDEC4-B7FF-4CF8-B924-57EF08E14A93}', '{7888A8AE-9EF7-453B-9EBF-93332A0C0B41}')
$registry = @('HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall', 'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall', 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall')
foreach ($root in $registry) {
    foreach ($key in $keys) {
        if (Test-Path -LiteralPath (Join-Path $root $key)) { throw 'TextureCraft MSI still registered' }
    }
}
if (Test-Path -LiteralPath 'C:\Program Files\TextureCraft\texturecraft.exe') { throw 'TextureCraft executable still present' }
if (Get-Process -Name texturecraft -ErrorAction SilentlyContinue) { throw 'TextureCraft still running' }
Write-Host 'QA_TEXTURECRAFT_REMOVED old and new MSI, executable and process absent'
