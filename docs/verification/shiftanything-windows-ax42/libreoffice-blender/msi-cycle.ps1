$ErrorActionPreference = 'Stop'
$qaRoot = 'C:\sq\win-office-qa'
$qaReceipts = Get-Content -LiteralPath (Join-Path $qaRoot 'preflight.json') -Raw | ConvertFrom-Json
if (@($qaReceipts | Where-Object Status -eq PASS).Count -ne 4) { throw 'Four exact passed MSI receipts required' }
$qaUser = 'ShiftOfficeQa'
function Get-QARegistry($item) {
    Get-ItemProperty -LiteralPath "HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\$($item.ProductCode)" -ErrorAction SilentlyContinue
}
function Get-QAPlan($app) {
    switch ($app) {
        'office' { @{
            items = @($qaReceipts | Where-Object Name -like 'LibreOffice_*.msi' | Sort-Object Name)
            exe = Join-Path $env:ProgramFiles 'LibreOffice\program\soffice.exe'
            args = '--writer --nologo --norestore'
            process = @('soffice','soffice.bin')
        } }
        'blender' { @{
            items = @($qaReceipts | Where-Object Name -like 'blender-*.msi' | Sort-Object Name)
            exe = Join-Path $env:ProgramFiles 'Blender Foundation\Blender 5.2\blender.exe'
            args = '--no-splash'
            process = @('blender')
        } }
        default { throw 'Unknown QA app' }
    }
}
function Start-QA {
    if (Get-LocalUser -Name $qaUser -ErrorAction SilentlyContinue) { throw 'Fixture account already exists' }
    foreach ($app in @('office','blender')) {
        $p = Get-QAPlan $app
        if ((Test-Path -LiteralPath $p.exe) -or ($p.items | Where-Object { Get-QARegistry $_ })) { throw 'Existing installation detected' }
    }
    $pw = ConvertTo-SecureString ([guid]::NewGuid().ToString('N') + '!a2') -AsPlainText -Force
    $u = New-LocalUser -Name $qaUser -Password $pw -Description 'Temporary isolated Windows vendor MSI QA'
    $members = Get-LocalGroup -SID ([Security.Principal.SecurityIdentifier]::new('S-1-5-32-545'))
    Add-LocalGroupMember -Group $members.Name -Member $u
    $a = Get-LocalGroup -SID ([Security.Principal.SecurityIdentifier]::new('S-1-5-32-544'))
    if ((Get-LocalGroupMember -Group $a.Name).SID -contains $u.SID) { throw 'Fixture privilege unexpected' }
    $script:qaCredential = [PSCredential]::new("$env:COMPUTERNAME\$qaUser", $pw)
    Write-Host "QA_STANDARD_ACCOUNT_READY $($u.Name)"
}
function Invoke-QAMsi($app, $phase) {
    $plan = Get-QAPlan $app
    $old, $new = $plan.items
    if ((Get-Process -Name $plan.process -ErrorAction SilentlyContinue)) { throw 'Close the fixture UI before changing installation' }
    switch ($phase) {
        'install' {
            if ((Get-QARegistry $old) -or (Get-QARegistry $new) -or (Test-Path -LiteralPath $plan.exe)) { throw 'Existing installation blocks fresh phase' }
            $current = $old
            $operation = 'i'
        }
        'update' {
            if (-not (Get-QARegistry $old) -or (Get-QARegistry $new)) { throw 'Older-only precondition missing' }
            $current = $new
            $operation = 'i'
        }
        'remove' {
            if ((Get-QARegistry $old) -or -not (Get-QARegistry $new)) { throw 'Newer-only precondition missing' }
            $current = $new
            $operation = 'x'
        }
        default { throw 'Unknown phase' }
    }
    $source = Join-Path (Join-Path $qaRoot 'msi') $current.Name
    if ((Get-FileHash -LiteralPath $source -Algorithm SHA256).Hash -ne $current.SHA256) { throw 'Installer hash changed' }
    $payload = if ($operation -eq 'i') { $source } else { $current.ProductCode }
    $log = Join-Path $qaRoot "$app-$phase.msi.log"
    $p = Start-Process msiexec.exe -ArgumentList @("/$operation", ("`"$payload`""), 'ALLUSERS=1', '/qn', '/norestart', '/l*v', ("`"$log`"")) -Wait -PassThru
    if ($p.ExitCode -ne 0) { throw "Windows Installer exit $($p.ExitCode); see $log" }
    $registry = Get-QARegistry $current
    if ($phase -eq 'remove') {
        if ($registry -or (Test-Path -LiteralPath $plan.exe)) { throw 'Uninstall left MSI registration or executable' }
        Write-Host "QA_NATIVE_MSI_PASS $app $phase $($current.Version)"
        return
    }
    if (-not $registry -or $registry.DisplayVersion -ne $current.Version -or $registry.Publisher -ne $current.Manufacturer -or -not (Test-Path -LiteralPath $plan.exe)) {
        Get-ChildItem -LiteralPath $env:ProgramFiles -ErrorAction SilentlyContinue | Where-Object Name -like '*Blender*' | Select-Object Name
        $registry | Select-Object DisplayName,DisplayVersion,Publisher,InstallLocation
        throw 'Installer did not produce expected registry and executable'
    }
    if ($phase -eq 'update' -and (Get-QARegistry $old)) { throw 'Old MSI remained registered after update' }
    Write-Host "QA_NATIVE_MSI_PASS $app $phase $($registry.DisplayName) | $($registry.DisplayVersion) | $($registry.Publisher)"
    Write-Host "QA_NATIVE_MSI_EXE $($plan.exe)"
}
function Open-QAApp($app) {
    $p = Get-QAPlan $app
    if (-not $script:qaCredential -or -not (Test-Path -LiteralPath $p.exe)) { throw 'Fixture or executable unavailable' }
    $launcher = Join-Path $PSScriptRoot 'launch-app.ps1'
    Start-Process powershell.exe -ArgumentList @('-NoProfile','-ExecutionPolicy','RemoteSigned','-File',$launcher,$app) -Credential $script:qaCredential -LoadUserProfile
}
function Confirm-QAOwner($app) {
    $fixture = Get-QAPlan $app
    $owned = Get-CimInstance Win32_Process | Where-Object Name -in @($fixture.process | ForEach-Object { if ($_ -match '\.') { $_ } else { "$_.exe" } }) | ForEach-Object {
        $owner = Invoke-CimMethod -InputObject $_ -MethodName GetOwner
        if ($_.ExecutablePath -and $owner.User -eq $qaUser) { $_.Name }
    }
    if (-not $owned) { throw 'Standard-user app process not found' }
    Write-Host "QA_STANDARD_PROCESS_OWNER $qaUser $($owned -join ',')"

}
function Read-QAProfile {
    $uid = (Get-LocalUser -Name $qaUser).SID
    $p = Get-CimInstance Win32_UserProfile | Where-Object SID -eq "$uid"
    $m = Join-Path $p.LocalPath 'AppData\Local\ShiftOfficeQa\profile-check.txt'
    if ((Get-Content -LiteralPath $m -Raw).Trim() -ne 'shift-office-qa-marker') { throw 'Synthetic profile marker missing' }
    Write-Host 'QA_STANDARD_PROFILE_PRESERVED'
}
function Clear-QAFixture {
    foreach ($app in @('office','blender')) {
        $plan = Get-QAPlan $app
        if ((Test-Path -LiteralPath $plan.exe) -or ($plan.items | Where-Object { Get-QARegistry $_ })) { throw 'Uninstall the fixture before cleanup' }
        if (Get-Process -Name $plan.process -ErrorAction SilentlyContinue) { throw 'Fixture application is running' }
    }
    $user = Get-LocalUser -Name $qaUser
    if ($user.Description -ne 'Temporary isolated Windows vendor MSI QA') { throw 'Unexpected fixture account' }
    $profile = Get-CimInstance Win32_UserProfile | Where-Object SID -eq "$($user.SID)"
    if ($profile.Loaded -or $profile.Special) { throw 'Fixture profile still loaded or special' }
    if ($profile) { $profile | Remove-CimInstance }
    Remove-LocalUser -SID $user.SID
    Remove-Item -LiteralPath (Join-Path $qaRoot 'msi') -Recurse -Force
    Remove-Variable qaCredential -Scope Script
    Write-Host 'QA_CLEANUP_PASS standard user, profile, MSI binaries; test logs retained'
}
