param([ValidateSet('empty', 'old', 'new', 'seed')] [string]$Phase = 'empty')
$ErrorActionPreference = 'Stop'
$qa = 'C:\sq\lightcraft-qa'
$parsed = Get-Content -LiteralPath (Join-Path $qa 'preflight.json') -Raw | ConvertFrom-Json
$receipts = @($parsed | Sort-Object Version)
if ($receipts.Count -ne 2 -or @($receipts | Where-Object Status -ne 'PASS').Count -ne 0) { throw 'Two valid official MSI receipts required' }
$old, $new = $receipts
$registry = @('HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall', 'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall', 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall')
function Get-Installation($Receipt) {
    foreach ($key in $registry) {
        Get-ItemProperty -LiteralPath (Join-Path $key $Receipt.ProductCode) -ErrorAction SilentlyContinue
    }
}
$exe = Join-Path $env:ProgramFiles 'ArtCraft\LightCraft\lightcraft.exe'
if ($Phase -in 'empty','seed') {
    if ((Get-Installation $old) -or (Get-Installation $new) -or (Test-Path -LiteralPath $exe) -or (Get-Process -Name lightcraft -ErrorAction SilentlyContinue)) { throw 'LightCraft fixture is not empty' }
    if ($Phase -eq 'empty') { Write-Host 'QA_LIGHTCRAFT_REMOVED both MSI registrations, executable and process absent'; return }
    $source = Join-Path (Join-Path $qa 'msi') $old.Name
    if ((Get-FileHash -LiteralPath $source).Hash -ne $old.SHA256 -or (Get-AuthenticodeSignature -FilePath $source).Status -ne 'Valid') { throw 'Archived old MSI invalid' }
    $task = Start-Process msiexec.exe -ArgumentList @('/i', $source, 'ALLUSERS=1', '/qn', '/norestart') -Wait -PassThru
    if ($task.ExitCode -ne 0) { throw "Update seed exit $($task.ExitCode)" }
    $Phase = 'old'
}
$expected = if ($Phase -eq 'old') { $old } else { $new }
$other = if ($Phase -eq 'old') { $new } else { $old }
$item = Get-Installation $expected
if ((Get-Installation $other) -or $item.DisplayVersion -ne $expected.Version -or $item.Publisher -ne $expected.Manufacturer -or -not (Test-Path -LiteralPath $exe)) { throw 'Unexpected MSI registration or executable' }
Write-Host "QA_LIGHTCRAFT_VERIFIED $($item.DisplayVersion) | $($item.Publisher) | $exe"
