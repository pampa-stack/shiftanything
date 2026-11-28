# Authenticode signing hook for Tauri's signCommand, used only by release-nsis.ps1 -Authenticode required.
# jsign (https://github.com/ebourg/jsign, Apache-2.0) reads the store password from the environment, never from arguments.
param([Parameter(Mandatory)] [string] $File)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

foreach ($name in 'SHIFT_WINDOWS_SIGN_JSIGN', 'SHIFT_WINDOWS_SIGN_STORETYPE', 'SHIFT_WINDOWS_SIGN_KEYSTORE', 'SHIFT_WINDOWS_SIGN_ALIAS', 'SHIFT_WINDOWS_SIGN_STOREPASS', 'SHIFT_WINDOWS_SIGN_TSAURL') {
  if (-not [Environment]::GetEnvironmentVariable($name)) { throw "$name is not set." }
}
$jar = Get-Item -LiteralPath $env:SHIFT_WINDOWS_SIGN_JSIGN
if ($jar.Extension -ne '.jar') { throw 'SHIFT_WINDOWS_SIGN_JSIGN must name the jsign JAR file.' }
& java.exe -jar $jar.FullName --storetype $env:SHIFT_WINDOWS_SIGN_STORETYPE --keystore $env:SHIFT_WINDOWS_SIGN_KEYSTORE `
  --alias $env:SHIFT_WINDOWS_SIGN_ALIAS --storepass 'env:SHIFT_WINDOWS_SIGN_STOREPASS' --alg SHA-256 `
  --tsaurl $env:SHIFT_WINDOWS_SIGN_TSAURL --tsmode RFC3161 --replace $File
if ($LASTEXITCODE -ne 0) { throw "jsign failed for $File (exit $LASTEXITCODE)." }
