# Build the Sh*ftAnything Windows x64 NSIS setup for a release, with Authenticode explicitly off or required.
# Run from a Visual Studio x64 developer prompt with Rust MSVC, the Windows SDK (signtool.exe) and the Tauri CLI.
# Details: packaging/RELEASE.md.
param(
  [Parameter(Mandatory)] [ValidatePattern('^[a-f0-9]{40}$')] [string] $SourceCommit,
  [Parameter(Mandatory)] [ValidateSet('off', 'required')] [string] $Authenticode,
  [Parameter(Mandatory)] [string] $OutputDirectory
)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Write-Utf8([string] $Path, [string] $Text) {
  # Windows PowerShell 5.1 writes a BOM with -Encoding utf8, which JSON readers reject.
  [IO.File]::WriteAllText($Path, $Text, [Text.UTF8Encoding]::new($false))
}

$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$app = Join-Path $root 'apps\shiftanything-tauri-proof'
if (Test-Path -LiteralPath $OutputDirectory) { throw 'Choose a new output directory.' }
if (-not [Environment]::Is64BitOperatingSystem -or $env:PROCESSOR_ARCHITECTURE -ne 'AMD64') { throw 'Build on Windows x64.' }

$metadata = & cargo.exe metadata --no-deps --format-version 1 --manifest-path (Join-Path $root 'Cargo.toml') | ConvertFrom-Json
if ($LASTEXITCODE -ne 0) { throw 'cargo metadata failed.' }
$cargoVersion = @($metadata.packages | Where-Object { $_.name -eq 'shiftanything-tauri-proof' })[0].version
$tauriVersion = (Get-Content -LiteralPath (Join-Path $app 'tauri.conf.json') -Raw | ConvertFrom-Json).version
if (-not $cargoVersion -or $cargoVersion -ne $tauriVersion) { throw "Cargo ($cargoVersion) and Tauri ($tauriVersion) versions differ." }
$version = $cargoVersion
# Windows does not allow * in file names, so tauri.windows.json names the Windows product and binary.
$windows = Get-Content -LiteralPath (Join-Path $app 'tauri.windows.json') -Raw | ConvertFrom-Json
$product = $windows.productName
if (-not $product -or $windows.mainBinaryName -ne $product) { throw 'tauri.windows.json must set productName and the same mainBinaryName.' }

# The launcher checks for updates only when both values are compiled in.
foreach ($name in 'SHIFT_LAUNCHER_UPDATE_ENDPOINT', 'SHIFT_LAUNCHER_UPDATE_PUBKEY') {
  if (-not [Environment]::GetEnvironmentVariable($name)) { throw "Set $name for the release update channel." }
}
# Updater artifacts are signed on the Mac that holds the key, never in this build.
foreach ($name in 'TAURI_SIGNING_PRIVATE_KEY', 'TAURI_SIGNING_PRIVATE_KEY_PATH', 'TAURI_SIGNING_PRIVATE_KEY_PASSWORD') {
  if ([Environment]::GetEnvironmentVariable($name)) { throw "Unset ${name}: updater signatures are made on the Mac." }
}

$signNames = 'SHIFT_WINDOWS_SIGN_JSIGN', 'SHIFT_WINDOWS_SIGN_JSIGN_SHA256', 'SHIFT_WINDOWS_SIGN_STORETYPE', 'SHIFT_WINDOWS_SIGN_KEYSTORE',
  'SHIFT_WINDOWS_SIGN_ALIAS', 'SHIFT_WINDOWS_SIGN_STOREPASS', 'SHIFT_WINDOWS_SIGN_TSAURL', 'SHIFT_WINDOWS_SIGN_SUBJECT'
$setNames = @(($signNames + 'SHIFT_WINDOWS_SIGN_KEYPASS') | Where-Object { [Environment]::GetEnvironmentVariable($_) })
if ($Authenticode -eq 'off') {
  if ($setNames.Count) { throw "Authenticode is off but $($setNames -join ', ') is set. Unset it or use -Authenticode required." }
} else {
  $missing = @($signNames | Where-Object { $_ -notin $setNames })
  if ($missing.Count) { throw "Authenticode is required but $($missing -join ', ') is not set." }
  Get-Command signtool.exe -CommandType Application | Out-Null
}

New-Item -ItemType Directory -Path $OutputDirectory | Out-Null
$output = (Resolve-Path -LiteralPath $OutputDirectory).Path
$bundleArgs = @('tauri', 'build', '--ci', '--target', 'x86_64-pc-windows-msvc', '--bundles', 'nsis', '--features', 'windows-lifecycle', '--config', 'tauri.windows.json')
if ($Authenticode -eq 'off') {
  $bundleArgs += '--no-sign'
} else {
  $signConfig = Join-Path $output 'tauri.authenticode.json'
  $config = @{ bundle = @{ windows = @{ signCommand = @{
    cmd = (Get-Command powershell.exe).Source
    args = @('-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'Bypass', '-File', (Join-Path $PSScriptRoot 'jsign-sign.ps1'), '%1')
  } } } } | ConvertTo-Json -Depth 6
  Write-Utf8 $signConfig $config
  $bundleArgs += @('--config', $signConfig)
}
$bundleArgs += @('--', '--locked')

Push-Location $app
try {
  & cargo.exe @bundleArgs
  if ($LASTEXITCODE -ne 0) { throw "Tauri build failed (exit $LASTEXITCODE)." }
} finally { Pop-Location }

# Cargo resolves a relative CARGO_TARGET_DIR against the directory it ran in.
$target = if (-not $env:CARGO_TARGET_DIR) { Join-Path $root 'target' }
  elseif ([IO.Path]::IsPathRooted($env:CARGO_TARGET_DIR)) { $env:CARGO_TARGET_DIR }
  else { Join-Path $app $env:CARGO_TARGET_DIR }
$release = Join-Path $target 'x86_64-pc-windows-msvc\release'
$built = Join-Path $release "bundle\nsis\${product}_${version}_x64-setup.exe"
$exe = Join-Path $release "$product.exe"
foreach ($file in $built, $exe) {
  $info = (Get-Item -LiteralPath $file).VersionInfo
  if ($info.ProductVersion -ne $version -or $info.ProductName -ne $product) { throw "Unexpected product or version: $file" }
}
$setup = Join-Path $output "$product-$version-windows-x64-setup.exe"
Copy-Item -LiteralPath $built -Destination $setup

if ($Authenticode -eq 'off') {
  foreach ($file in $setup, $exe) {
    $status = (Get-AuthenticodeSignature -LiteralPath $file).Status
    if ($status -ne 'NotSigned') { throw "Expected no Authenticode signature with signing off: $file ($status)" }
  }
  Write-Output 'UNSIGNED: Authenticode is off. SmartScreen may warn on first run, and Smart App Control blocks unsigned setups.'
} else {
  & (Join-Path $PSScriptRoot 'verify-release-signature.ps1') -Artifact @($setup, $exe) -PublisherSubject $env:SHIFT_WINDOWS_SIGN_SUBJECT | Out-Null
  Write-Output 'SIGNED: Authenticode verified on the setup and the launcher executable.'
}

$hash = (Get-FileHash -LiteralPath $setup -Algorithm SHA256).Hash.ToLowerInvariant()
$receipt = [ordered]@{
  version = $version
  commit = $SourceCommit
  product = $product
  file = (Split-Path -Leaf $setup)
  size = (Get-Item -LiteralPath $setup).Length
  sha256 = $hash
  authenticode = $Authenticode
} | ConvertTo-Json
Write-Utf8 (Join-Path $output 'windows-receipt.json') $receipt
Write-Output "PASS $hash  $(Split-Path -Leaf $setup)"
