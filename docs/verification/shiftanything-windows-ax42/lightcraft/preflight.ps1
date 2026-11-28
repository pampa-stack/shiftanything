$ErrorActionPreference = 'Stop'
$root = 'C:\sq\lightcraft-qa'
$msi = Join-Path $root 'msi'
New-Item -ItemType Directory -Path $msi -Force | Out-Null
if ((Get-PSDrive C).Free -lt 20GB) { throw 'Guest disk reserve is insufficient' }
$assets = Import-Csv (Join-Path $PSScriptRoot 'receipts.csv')
$ins = New-Object -ComObject WindowsInstaller.Installer
$invoke = [Reflection.BindingFlags]::InvokeMethod
$property = [Reflection.BindingFlags]::GetProperty
$kitRoot = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits\10\bin'
$signTool = Get-ChildItem -LiteralPath $kitRoot -Filter signtool.exe -File -Recurse | Where-Object DirectoryName -like '*\x64' | Sort-Object FullName -Descending | Select-Object -First 1
if (-not $signTool) { throw 'Windows SDK x64 SignTool is unavailable' }
$results = foreach ($asset in $assets) {
    $path = Join-Path $msi $asset.name
    try {
        if (-not (Test-Path -LiteralPath $path)) {
            Copy-Item -LiteralPath (Join-Path $PSScriptRoot $asset.name) -Destination "$path.part"
            if ((Get-Item -LiteralPath "$path.part").Length -ne [long]$asset.bytes -or (Get-FileHash -LiteralPath "$path.part" -Algorithm SHA256).Hash -ne $asset.sha256) { throw 'Size or SHA-256 mismatch' }
            Move-Item -LiteralPath "$path.part" -Destination $path
        }
        if ((Get-Item -LiteralPath $path).Length -ne [long]$asset.bytes -or (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash -ne $asset.sha256) { throw 'Stored package receipt mismatch' }
        $sig = Get-AuthenticodeSignature -LiteralPath $path
        $publisher = $sig.SignerCertificate.GetNameInfo('SimpleName', $false)
        if ($sig.Status -ne 'Valid' -or -not $sig.TimeStamperCertificate -or $publisher -notin ($asset.publisher -split '\|')) { throw "Untrusted publisher or timestamp: $($sig.Status) signing publisher $publisher" }
        & $signTool.FullName verify /pa /all /tw $path | Set-Content -LiteralPath (Join-Path $root "$($asset.name).signtool.txt")
        if ($LASTEXITCODE -ne 0) { throw "SignTool exit $LASTEXITCODE" }
        Write-Host "MSI_SIGNATURE_PASS $($asset.name) | $publisher | $($sig.SignerCertificate.Subject)"
        $item = Get-Item -LiteralPath $path
        $db = $ins.GetType().InvokeMember('OpenDatabase', $invoke, $null, $ins, @($item.FullName, 0))
        $view = $db.GetType().InvokeMember('OpenView', $invoke, $null, $db, @('SELECT * FROM Property'))
        $null = $view.GetType().InvokeMember('Execute', $invoke, $null, $view, $null)
        $metadata = @{}
        while ($record = $view.GetType().InvokeMember('Fetch', $invoke, $null, $view, $null)) {
            $metadata[$record.GetType().InvokeMember('StringData', $property, $null, $record, @(1))] = $record.GetType().InvokeMember('StringData', $property, $null, $record, @(2))
        }
        $summary = $db.GetType().InvokeMember('SummaryInformation', $property, $null, $db, @(0))
        $template = $summary.GetType().InvokeMember('Property', $property, $null, $summary, @(7))
        if ($template -notmatch '^(x64|Intel64);' -or $metadata.ProductVersion -cne $asset.msi_version) { throw "MSI architecture or version mismatch ($template, $($metadata.ProductVersion))" }
        Write-Host "MSI_PREFLIGHT_PASS $($asset.name) | $publisher | $($metadata.ProductName) | $($metadata.Manufacturer) | $($metadata.ProductVersion) | $template | scope $($metadata.ALLUSERS)"
        [PSCustomObject]@{ Name=$asset.name; Version=$metadata.ProductVersion; ProductCode=$metadata.ProductCode; Manufacturer=$metadata.Manufacturer; ProductName=$metadata.ProductName; Template=$template; Publisher=$publisher; SHA256=$asset.sha256; Status='PASS' }
    } catch {
        Write-Host "MSI_PREFLIGHT_REFUSED $($asset.name) $($_.InvocationInfo.ScriptLineNumber) $($_.Exception.Message)"
        [PSCustomObject]@{ Name=$asset.name; Status='REFUSED'; Error=$_.Exception.Message }
    } finally {
        if (Test-Path -LiteralPath "$path.part") { Remove-Item -LiteralPath "$path.part" -Force }
    }
}
$results | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $root 'preflight.json')
Write-Host "MSI_PREFLIGHT_TOTAL $(@($results | Where-Object Status -eq 'PASS').Count)/$($assets.Count)"
