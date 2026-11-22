@echo off
rem vhnoterPDF office setup: installs vhnoterPDF for this Windows user, the office's Albanian OCR
rem word list and Tesseract (the OCR engine for Albanian). Run it from the deploy folder; it needs
rem no arguments. Windows asks for approval once, for Tesseract only. Safe to run again: an
rem installed vhnoterPDF is upgraded in place and an installed Tesseract is kept.
setlocal
cd /d "%~dp0"

set "MSI="
for %%f in (vhnoterpdf-*-windows-x64.msi) do set "MSI=%%f"
if not defined MSI (
  echo No vhnoterpdf-*-windows-x64.msi in %~dp0
  exit /b 1
)

echo Installing %MSI% ...
msiexec /i "%MSI%" /qb- /norestart
if errorlevel 3010 goto msi_ok
if errorlevel 1 (
  echo vhnoterPDF was not installed ^(msiexec error %errorlevel%^).
  exit /b %errorlevel%
)
:msi_ok

if exist "ocr-dictionary-sq-office.txt" (
  echo Copying the office word list ...
  if not exist "%LOCALAPPDATA%\vhnoterPDF\Dictionary" mkdir "%LOCALAPPDATA%\vhnoterPDF\Dictionary"
  copy /y "ocr-dictionary-sq-office.txt" "%LOCALAPPDATA%\vhnoterPDF\Dictionary\" >nul
)

if exist "%ProgramFiles%\Tesseract-OCR\tesseract.exe" (
  echo Tesseract is already installed.
  goto done
)
if exist "%LOCALAPPDATA%\Programs\Tesseract-OCR\tesseract.exe" (
  echo Tesseract is already installed.
  goto done
)
set "TESS="
for %%f in (tesseract-ocr-w64-setup-*.exe) do set "TESS=%%f"
if not defined TESS (
  echo No Tesseract installer here: vhnoterPDF offers OCR ^> Install Tesseract instead.
  goto done
)
echo Installing Tesseract ^(approve the Windows prompt^) ...
start "" /wait "%TESS%" /S
if not exist "%ProgramFiles%\Tesseract-OCR\tesseract.exe" echo Tesseract did not install: run %TESS% by hand.

:done
echo Done. Start vhnoterPDF from the Start menu.
endlocal
