param([ValidateSet('office','blender')] [string]$App)
$ErrorActionPreference = 'Stop'
$id = [Security.Principal.WindowsIdentity]::GetCurrent()
if ([Security.Principal.WindowsPrincipal]::new($id).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw 'App QA process must be standard user'
}
$folder = Join-Path $env:LOCALAPPDATA 'ShiftOfficeQa'
New-Item $folder -ItemType Directory -Force | Out-Null
$marker = Join-Path $folder 'profile-check.txt'
if (-not (Test-Path $marker)) { Set-Content -LiteralPath $marker -Value 'shift-office-qa-marker' }
switch ($App) {
    'office' { Start-Process (Join-Path $env:ProgramFiles 'LibreOffice\program\soffice.exe') -ArgumentList '--writer --nologo --norestore' }
    'blender' { Start-Process (Join-Path $env:ProgramFiles 'Blender Foundation\Blender 5.2\blender.exe') -ArgumentList '--no-splash' }
}
