$ErrorActionPreference = 'Stop'
$project = 'C:\sq\src-8777fda-lightcraft'
$stage = Join-Path $env:PUBLIC 'Documents\ShiftLightCandidate'
$local = 'C:\sq\lightcraft-candidate-build'
if ((Get-PSDrive C).Free -lt 20GB -or (Test-Path $project) -or (Test-Path $stage) -or (Test-Path $local)) { throw 'Guest disk reserve or fixture precondition failed' }
$rows = Import-Csv (Join-Path $PSScriptRoot 'source-sha.csv')
foreach ($asset in $rows) {
    if ((Get-FileHash (Join-Path $PSScriptRoot $asset.name) -Algorithm SHA256).Hash -ne $asset.sha256) { throw 'Source receipt SHA mismatch' }
}
New-Item -Path $project,$local -ItemType Directory | Out-Null
& tar.exe -xf (Join-Path $PSScriptRoot 'src.tar.gz') -C $project
if ($LASTEXITCODE -ne 0) { throw 'Archive extraction failed' }
$targets = @{
    'release.rs' = 'crates\core\src\release.rs'
    'platform.rs' = 'crates\core\src\platform.rs'
    'verified-platforms.json' = 'apps\shiftanything-tauri-proof\verified-platforms.json'
    'session.rs' = 'apps\shiftanything-tauri-proof\src\session.rs'
}
foreach ($pair in $targets.GetEnumerator()) {
    $src = Join-Path $PSScriptRoot $pair.Key
    $dst = Join-Path $project $pair.Value
    Copy-Item -LiteralPath $src -Destination $dst -Force
    (Get-Item -LiteralPath $dst).IsReadOnly = $false
    if ((Get-FileHash $dst).Hash -ne (Get-FileHash $src).Hash) { throw 'Source overlay SHA mismatch' }
}
$cmd = Join-Path $local 'build.cmd'
$log = Join-Path $local 'build.log'
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'build.cmd') -Destination $cmd
& cmd.exe /c "$cmd > $log 2>&1"
if ($LASTEXITCODE -ne 0 -or (Get-Content $log -Tail 10) -notcontains 'CANDIDATE_BUILD_PASS') {
    Get-Content $log -Tail 30
    throw 'Candidate build did not pass'
}
Select-String -Path $log -Pattern 'windows_x64_selects_only_reviewed_msi_receipts|test result:|Finished|CANDIDATE_BUILD_PASS' | ForEach-Object Line
$out = 'C:\sq\target\x86_64-pc-windows-msvc\release\shiftanything-tauri-proof.exe'
if ((Get-Item -LiteralPath $out).VersionInfo.ProductVersion -notlike '0.20.9*') { throw 'Unexpected candidate version' }
New-Item -Path $stage -ItemType Directory | Out-Null
$exe = Join-Path $stage 'ShiftAnything.exe'
Copy-Item -LiteralPath $out -Destination $exe
if ((Get-FileHash -LiteralPath $out).Hash -ne (Get-FileHash -LiteralPath $exe).Hash) { throw 'Candidate staging hash mismatch' }
Get-FileHash -LiteralPath $exe | Format-List
Get-Item -LiteralPath $exe | Format-List Length,@{Label='Version';Expression={$_.VersionInfo.ProductVersion}}
Write-Host 'LIGHTCRAFT_CANDIDATE_STAGED'
