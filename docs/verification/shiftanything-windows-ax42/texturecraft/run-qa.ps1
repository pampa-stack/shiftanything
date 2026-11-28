$ErrorActionPreference = 'Stop'
$root = 'C:\sq\texturecraft-qa'
& (Join-Path $PSScriptRoot 'preflight.ps1')
$qaReceipts = Get-Content -LiteralPath (Join-Path $root 'preflight.json') -Raw | ConvertFrom-Json
if (@($qaReceipts | Where-Object Status -eq PASS).Count -ne 2) { throw 'Both upstream MSIs must pass' }
$qaReceipts | Select-Object Version, ProductName, Manufacturer, Publisher, ProductCode | Format-Table -AutoSize
foreach ($name in @('cycle.ps1','launch-app.ps1')) {
    $dest = Join-Path $root $name
    Copy-Item -LiteralPath (Join-Path $PSScriptRoot $name) -Destination $dest -Force
    (Get-Item -LiteralPath $dest).IsReadOnly = $false
}
. (Join-Path $root 'cycle.ps1')
