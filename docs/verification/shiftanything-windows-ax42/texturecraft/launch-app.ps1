param([ValidateSet('texturecraft')] [string]$App)
$ErrorActionPreference = 'Stop'
$id = [Security.Principal.WindowsIdentity]::GetCurrent()
if ([Security.Principal.WindowsPrincipal]::new($id).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw 'App QA process must be standard user'
}
$folder = Join-Path $env:LOCALAPPDATA 'ShiftTextureQa'
New-Item $folder -ItemType Directory -Force | Out-Null
$marker = Join-Path $folder 'profile-check.txt'
if (-not (Test-Path $marker)) { Set-Content -LiteralPath $marker -Value 'shift-texture-qa-marker' }
Start-Process (Join-Path $env:ProgramFiles 'TextureCraft\texturecraft.exe')
