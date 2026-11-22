<#
.SYNOPSIS
  Build the vhnoterPDF office deploy folder: dist\deploy\ with the MSI, the official Tesseract
  installer, the office's Albanian OCR word list and deploy.cmd, which installs all three.
.DESCRIPTION
  Run after package.ps1. Tesseract stays its own installer (its Windows build carries LGPL and GPL
  runtime libraries, so it is never put inside vhnoterPDF's MSI); it is downloaded from its
  official release and checked against a pinned SHA-256. The office word list is built from the
  private app.vhnoter.al templates (beside this repository, or VHNOTER_TEMPLATES); without them the
  folder simply has no office list. The folder is private: copy it to the office PCs only, never
  publish it.
.EXAMPLE
  powershell -File packaging/windows/deploy.ps1
#>
param([string] $Arch = 'x64')
$ErrorActionPreference = 'Stop'
$Root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$Release = Join-Path $Root 'dist\release'
$Deploy = Join-Path $Root 'dist\deploy'
$Cache = Join-Path $Root 'target\deploy-cache'

$TesseractUrl = 'https://github.com/UB-Mannheim/tesseract/releases/download/v5.4.0.20240606/tesseract-ocr-w64-setup-5.4.0.20240606.exe'
$TesseractSha256 = 'c885fff6998e0608ba4bb8ab51436e1c6775c2bafc2559a19b423e18678b60c9'

$version = (Select-String -Path (Join-Path $Root 'Cargo.toml') -Pattern '^version = "([^"]+)"' | Select-Object -First 1).Matches[0].Groups[1].Value
$msi = Join-Path $Release "vhnoterpdf-$version-windows-$Arch.msi"
if (-not (Test-Path -LiteralPath $msi)) { throw "No ${msi}: run packaging/windows/package.ps1 first" }

if (Test-Path -LiteralPath $Deploy) { Remove-Item -Recurse -Force -LiteralPath $Deploy }
New-Item -ItemType Directory -Force -Path $Deploy, $Cache | Out-Null
Copy-Item -LiteralPath $msi -Destination $Deploy
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'deploy.cmd') -Destination $Deploy

# Tesseract: the official installer, downloaded once and verified every time.
$tesseract = Join-Path $Cache (Split-Path $TesseractUrl -Leaf)
if (-not (Test-Path -LiteralPath $tesseract) -or (Get-FileHash -Algorithm SHA256 -LiteralPath $tesseract).Hash -ne $TesseractSha256) {
  Write-Host "Downloading $TesseractUrl"
  & curl.exe --fail --location --silent --show-error --output $tesseract $TesseractUrl
  if ($LASTEXITCODE -ne 0) { throw "Downloading Tesseract failed ($LASTEXITCODE)" }
}
$hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $tesseract).Hash
if ($hash -ne $TesseractSha256) { throw "Tesseract installer SHA-256 $hash, expected $TesseractSha256" }
Copy-Item -LiteralPath $tesseract -Destination $Deploy

# The office word list: template words, with the public list as reference for e-less spellings.
$templates = if ($env:VHNOTER_TEMPLATES) { $env:VHNOTER_TEMPLATES } else { Join-Path $Root 'app.vhnoter.al\js\data\documents' }
$frequencies = Join-Path $Root 'assets\models\sq_50k.txt'
if (Test-Path -LiteralPath $templates) {
  Push-Location $Root
  try {
    & cargo xtask models | Out-Null
    if ($LASTEXITCODE -ne 0) { throw "cargo xtask models failed ($LASTEXITCODE)" }
    & cargo xtask ocr-dictionary (Join-Path $Deploy 'ocr-dictionary-sq-office.txt') --reference $frequencies $templates
    if ($LASTEXITCODE -ne 0) { throw "cargo xtask ocr-dictionary failed ($LASTEXITCODE)" }
  } finally { Pop-Location }
} else {
  Write-Host "No office templates at ${templates}: the deploy folder has no office word list."
}

Write-Host "Deploy folder: $Deploy"
Get-ChildItem -LiteralPath $Deploy | ForEach-Object { '  {0}  {1:N1} MB' -f $_.Name, ($_.Length / 1MB) }
