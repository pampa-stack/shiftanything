# Authenticode signing hook for Tauri's signCommand, used only by release-nsis.ps1 -Authenticode required.
# jsign (https://github.com/ebourg/jsign, Apache-2.0) runs only when its JAR matches the pinned SHA-256, and the
# store password reaches it through a private temporary file: it is neither in arguments nor in Java's environment.
param([Parameter(Mandatory)] [string] $File)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

foreach ($name in 'SHIFT_WINDOWS_SIGN_JSIGN', 'SHIFT_WINDOWS_SIGN_JSIGN_SHA256', 'SHIFT_WINDOWS_SIGN_STORETYPE', 'SHIFT_WINDOWS_SIGN_KEYSTORE',
  'SHIFT_WINDOWS_SIGN_ALIAS', 'SHIFT_WINDOWS_SIGN_STOREPASS', 'SHIFT_WINDOWS_SIGN_TSAURL') {
  if (-not [Environment]::GetEnvironmentVariable($name)) { throw "$name is not set." }
}
$jar = Get-Item -LiteralPath $env:SHIFT_WINDOWS_SIGN_JSIGN
if ($jar.Extension -ne '.jar') { throw 'SHIFT_WINDOWS_SIGN_JSIGN must name the jsign JAR file.' }
$expected = $env:SHIFT_WINDOWS_SIGN_JSIGN_SHA256.ToLowerInvariant()
if ($expected -notmatch '^[0-9a-f]{64}$') { throw 'SHIFT_WINDOWS_SIGN_JSIGN_SHA256 must be the SHA-256 of the official jsign JAR.' }
if ((Get-FileHash -LiteralPath $jar.FullName -Algorithm SHA256).Hash.ToLowerInvariant() -ne $expected) {
  throw 'The jsign JAR does not match SHIFT_WINDOWS_SIGN_JSIGN_SHA256.'
}

# Move the secrets out of this process environment before starting Java.
$secrets = [ordered]@{ storepass = $env:SHIFT_WINDOWS_SIGN_STOREPASS; keypass = $env:SHIFT_WINDOWS_SIGN_KEYPASS }
Remove-Item Env:SHIFT_WINDOWS_SIGN_STOREPASS
Remove-Item Env:SHIFT_WINDOWS_SIGN_KEYPASS -ErrorAction SilentlyContinue
$private = Join-Path ([IO.Path]::GetTempPath()) ("shift-jsign-" + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $private | Out-Null
try {
  # Only the current user may read the password files.
  $acl = New-Object Security.AccessControl.DirectorySecurity
  $acl.SetAccessRuleProtection($true, $false)
  $acl.AddAccessRule((New-Object Security.AccessControl.FileSystemAccessRule(
    [Security.Principal.WindowsIdentity]::GetCurrent().User, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow')))
  Set-Acl -LiteralPath $private -AclObject $acl
  $arguments = @('-jar', $jar.FullName, '--storetype', $env:SHIFT_WINDOWS_SIGN_STORETYPE, '--keystore', $env:SHIFT_WINDOWS_SIGN_KEYSTORE,
    '--alias', $env:SHIFT_WINDOWS_SIGN_ALIAS, '--alg', 'SHA-256', '--tsaurl', $env:SHIFT_WINDOWS_SIGN_TSAURL, '--tsmode', 'RFC3161', '--replace')
  foreach ($kind in $secrets.Keys) {
    if (-not $secrets[$kind]) { continue }
    $path = Join-Path $private $kind
    [IO.File]::WriteAllText($path, $secrets[$kind], [Text.UTF8Encoding]::new($false))
    $arguments += @("--$kind", "file:$path")
  }
  & java.exe @arguments $File
  if ($LASTEXITCODE -ne 0) { throw "jsign failed for $File (exit $LASTEXITCODE)." }
} finally {
  Remove-Item -LiteralPath $private -Recurse -Force -ErrorAction SilentlyContinue
}
