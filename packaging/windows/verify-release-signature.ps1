param(
  [Parameter(Mandatory)] [string[]] $Artifact,
  [Parameter(Mandatory)] [ValidateNotNullOrEmpty()] [string] $PublisherSubject,
  [string] $SignToolPath = 'signtool.exe'
)
$ErrorActionPreference = 'Stop'

$signTool = (Get-Command -Name $SignToolPath -CommandType Application).Source
foreach ($path in $Artifact) {
  $file = Get-Item -LiteralPath $path
  if ($file.PSIsContainer -or $file.Extension -notin '.exe', '.msi') {
    throw "Expected a Windows executable or installer: $path"
  }

  $signature = Get-AuthenticodeSignature -LiteralPath $file.FullName
  if ($signature.Status -ne 'Valid' -or $signature.SignatureType -ne 'Authenticode') {
    throw "A valid embedded Authenticode signature is required: $($file.Name) ($($signature.Status))"
  }
  if ($signature.SignerCertificate.Subject -ne $PublisherSubject) {
    throw "Unexpected signing publisher: $($file.Name)"
  }
  if (-not $signature.TimeStamperCertificate) {
    throw "A trusted timestamp is required: $($file.Name)"
  }

  & $signTool verify /pa /all /tw $file.FullName
  if ($LASTEXITCODE -ne 0) { throw "SignTool rejected $($file.Name) (exit $LASTEXITCODE)" }
  Get-FileHash -LiteralPath $file.FullName -Algorithm SHA256
}
