# Build the Sh*ftAnything Windows x64 NSIS setup for a release, with Authenticode explicitly off or required.
# Run from a Visual Studio x64 developer prompt with Rust MSVC and the Tauri CLI. Details: packaging/RELEASE.md.
param(
  [Parameter(Mandatory)] [ValidatePattern('^[a-f0-9]{40}$')] [string] $SourceCommit,
  [Parameter(Mandatory)] [ValidateSet('off', 'required')] [string] $Authenticode,
  [Parameter(Mandatory)] [string] $OutputDirectory
)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$app = Join-Path $root 'apps\shiftanything-tauri-proof'
if (Test-Path -LiteralPath $OutputDirectory) { throw 'Choose a new output directory.' }
if (-not [Environment]::Is64BitOperatingSystem -or $env:PROCESSOR_ARCHITECTURE -ne 'AMD64') { throw 'Build on Windows x64.' }

$cargoVersion = (Select-String -LiteralPath (Join-Path $root 'Cargo.toml') -Pattern '^version = "([0-9]+\.[0-9]+\.[0-9]+)"$' | Select-Object -First 1).Matches[0].Groups[1].Value
$tauriVersion = (Get-Content -LiteralPath (Join-Path $app 'tauri.conf.json') -Raw | ConvertFrom-Json).version
if (-not $cargoVersion -or $cargoVersion -ne $tauriVersion) { throw "Cargo ($cargoVersion) and Tauri ($tauriVersion) versions differ." }
$version = $cargoVersion

# The launcher checks for updates only when both values are compiled in.
foreach ($name in 'SHIFT_LAUNCHER_UPDATE_ENDPOINT', 'SHIFT_LAUNCHER_UPDATE_PUBKEY') {
  if (-not [Environment]::GetEnvironmentVariable($name)) { throw "Set $name for the release update channel." }
}
# Updater artifacts are signed on the Mac that holds the key, never in this build.
foreach ($name in 'TAURI_SIGNING_PRIVATE_KEY', 'TAURI_SIGNING_PRIVATE_KEY_PATH', 'TAURI_SIGNING_PRIVATE_KEY_PASSWORD') {
  if ([Environment]::GetEnvironmentVariable($name)) { throw "Unset ${name}: updater signatures are made on the Mac." }
}

$signNames = 'SHIFT_WINDOWS_SIGN_JSIGN', 'SHIFT_WINDOWS_SIGN_STORETYPE', 'SHIFT_WINDOWS_SIGN_KEYSTORE', 'SHIFT_WINDOWS_SIGN_ALIAS',
  'SHIFT_WINDOWS_SIGN_STOREPASS', 'SHIFT_WINDOWS_SIGN_TSAURL', 'SHIFT_WINDOWS_SIGN_SUBJECT'
$setNames = @($signNames | Where-Object { [Environment]::GetEnvironmentVariable($_) })
New-Item -ItemType Directory -Path $OutputDirectory | Out-Null
$output = (Resolve-Path -LiteralPath $OutputDirectory).Path
$bundleArgs = @('tauri', 'build', '--ci', '--target', 'x86_64-pc-windows-msvc', '--bundles', 'nsis', '--features', 'windows-lifecycle', '--config', 'tauri.windows.json')
if ($Authenticode -eq 'off') {
  if ($setNames.Count) { throw "Authenticode is off but $($setNames -join ', ') is set. Unset it or use -Authenticode required." }
  $bundleArgs += '--no-sign'
} else {
  $missing = @($signNames | Where-Object { $_ -notin $setNames })
  if ($missing.Count) { throw "Authenticode is required but $($missing -join ', ') is not set." }
  $signConfig = Join-Path $output 'tauri.authenticode.json'
  @{ bundle = @{ windows = @{ signCommand = @{
    cmd = (Get-Command powershell.exe).Source
    args = @('-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'Bypass', '-File', (Join-Path $PSScriptRoot 'jsign-sign.ps1'), '%1')
  } } } } | ConvertTo-Json -Depth 6 | Set-Content -LiteralPath $signConfig -Encoding utf8
  $bundleArgs += @('--config', $signConfig)
}
$bundleArgs += @('--', '--locked')

Push-Location $app
try {
  & cargo.exe @bundleArgs
  if ($LASTEXITCODE -ne 0) { throw "Tauri build failed (exit $LASTEXITCODE)." }
} finally { Pop-Location }

$target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $root 'target' }
$release = Join-Path $target 'x86_64-pc-windows-msvc\release'
$built = Join-Path $release "bundle\nsis\ShiftAnything_${version}_x64-setup.exe"
$exe = Join-Path $release 'ShiftAnything.exe'
foreach ($file in $built, $exe) {
  $info = (Get-Item -LiteralPath $file).VersionInfo
  if (-not $info.ProductVersion.StartsWith($version) -or $info.ProductName -ne 'ShiftAnything') { throw "Unexpected product or version: $file" }
}
$setup = Join-Path $output "ShiftAnything-$version-windows-x64-setup.exe"
Copy-Item -LiteralPath $built -Destination $setup

if ($Authenticode -eq 'off') {
  foreach ($file in $setup, $exe) {
    $status = (Get-AuthenticodeSignature -LiteralPath $file).Status
    if ($status -ne 'NotSigned') { throw "Expected no Authenticode signature with signing off: $file ($status)" }
  }
  Write-Host 'UNSIGNED: Authenticode is off. Windows SmartScreen will show "Windows protected your PC" on first run.'
} else {
  & (Join-Path $PSScriptRoot 'verify-release-signature.ps1') -Artifact @($setup, $exe) -PublisherSubject $env:SHIFT_WINDOWS_SIGN_SUBJECT | Out-Null
  Write-Host 'SIGNED: Authenticode verified on the setup and the launcher executable.'
}

$hash = (Get-FileHash -LiteralPath $setup -Algorithm SHA256).Hash.ToLowerInvariant()
[ordered]@{
  version = $version
  commit = $SourceCommit
  file = (Split-Path -Leaf $setup)
  size = (Get-Item -LiteralPath $setup).Length
  sha256 = $hash
  authenticode = $Authenticode
} | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $output 'windows-receipt.json') -Encoding utf8
Write-Host "PASS $hash  $(Split-Path -Leaf $setup)"
