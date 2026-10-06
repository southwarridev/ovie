@echo off
REM ============================================================================
REM  Ovie Programming Language v2.3.0 — Windows Batch Installer
REM  Works by downloading prebuilt windows-x64 binaries from GitHub releases.
REM  No Rust, no manual cloning needed.
REM ============================================================================

setlocal enabledelayedexpansion

set "OVIE_VERSION=2.3.0"
set "INSTALL_DIR=C:\Program Files\Ovie"
set "BIN_DIR=%INSTALL_DIR%\bin"
set "GITHUB_REPO=southwarridev/ovie"

echo.
echo   ============================================================================
echo   ^|                                                                          ^|
echo   ^|              OVIE PROGRAMMING LANGUAGE v2.3.0                           ^|
echo   ^|              Complete Module System - Full Package                       ^|
echo   ^|              Publisher: Ovie Language Team  |  MIT License              ^|
echo   ^|                                                                          ^|
echo   ============================================================================
echo.

REM ── Check for Admin ──────────────────────────────────────────────────────────
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo   [WARN] Not running as Administrator.
    echo   [WARN] Please right-click this file and choose "Run as administrator".
    echo.
    pause
    exit /b 1
)
echo   Running as Administrator: YES
echo.

REM ── Step 1: Download latest release ──────────────────────────────────────────
echo   [1/3] Downloading latest Ovie release...

set "API_URL=https://api.github.com/repos/%GITHUB_REPO%/releases/latest"
set "ASSET=ovie-windows-x64.zip"
set "DOWNLOAD_URL="

REM Fetch release info and extract download URL
powershell -Command ^
    "$resp = Invoke-RestMethod -Uri 'https://api.github.com/repos/%GITHUB_REPO%/releases/latest' -Headers @{'Accept'='application/vnd.github.v3+json'}; " ^
    "foreach ($asset in $resp.assets) { if ($asset.name -eq '%ASSET%') { $asset.browser_download_url; exit } }" > "%TEMP%\download_url.txt" 2>nul

set /p DOWNLOAD_URL=<"%TEMP%\download_url.txt"

if not defined DOWNLOAD_URL (
    echo   [ERROR] Could not find download URL for %ASSET%
    echo   [ERROR] Check your internet connection or GitHub availability.
    pause
    exit /b 1
)

echo   [OK] Download URL found: %DOWNLOAD_URL%

REM Download the archive
powershell -Command ^
    "Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%TEMP%\%ASSET%' -UseBasicParsing"

if %errorlevel% neq 0 (
    echo   [ERROR] Download failed. Check your internet connection.
    pause
    exit /b 1
)

echo   [OK] Downloaded %ASSET%

REM ── Step 2: Extract and copy binaries ────────────────────────────────────────
echo.
echo   [2/3] Installing Ovie...

REM Create directories
for %%D in ("%INSTALL_DIR%" "%BIN_DIR%" "%INSTALL_DIR%\std" "%INSTALL_DIR%\examples" "%INSTALL_DIR%\docs") do (
    if not exist "%%~D" mkdir "%%~D" >nul 2>&1
)

REM Extract the archive (using PowerShell for reliability)
powershell -Command ^
    "Expand-Archive -Path '%TEMP%\%ASSET%' -DestinationPath '%TEMP%\ovie-installer' -Force"

REM Copy binaries
copy /y "%TEMP%\ovie-installer\ovie\bin\oviec.exe" "%BIN_DIR%\oviec.exe" >nul
copy /y "%TEMP%\ovie-installer\ovie\bin\ovie.exe" "%BIN_DIR%\ovie.exe" >nul 2>nul

echo   [OK] Binaries installed

REM Copy stdlib, examples, docs
if exist "%TEMP%\ovie-installer\ovie\std" (
    xcopy /e /y /q "%TEMP%\ovie-installer\ovie\std\*" "%INSTALL_DIR%\std\" >nul
)
if exist "%TEMP%\ovie-installer\ovie\examples" (
    xcopy /e /y /q "%TEMP%\ovie-installer\ovie\examples\*" "%INSTALL_DIR%\examples\" >nul
)
if exist "%TEMP%\ovie-installer\ovie\docs" (
    xcopy /e /y /q "%TEMP%\ovie-installer\ovie\docs\*" "%INSTALL_DIR%\docs\" >nul
)

REM Copy root files
for %%F in (README.md LICENSE) do (
    if exist "%TEMP%\ovie-installer\ovie\%%F" copy /y "%TEMP%\ovie-installer\ovie\%%F" "%INSTALL_DIR%\%%F" >nul
)

echo   [OK] Standard library and examples copied

REM ── Step 3: Add to PATH ───────────────────────────────────────────────────────
echo.
echo   [3/3] Adding %BIN_DIR% to system PATH...
REM Read current system PATH from registry
for /f "usebackq tokens=2,*" %%A in (`reg query "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment" /v Path 2^>nul`) do set "CURRENT_PATH=%%B"

echo !CURRENT_PATH! | findstr /i /c:"%BIN_DIR%" >nul
if %errorlevel% neq 0 (
    reg add "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment" /v Path /t REG_EXPAND_SZ /d "!CURRENT_PATH!;%BIN_DIR%" /f >nul 2>&1
    echo   [OK] Added to system PATH
) else (
    echo   [OK] Already in system PATH
)

REM ── Cleanup ───────────────────────────────────────────────────────────────────
echo.
echo   Cleaning up temporary files...
if exist "%TEMP%\ovie-installer" rmdir /s /q "%TEMP%\ovie-installer" >nul 2>&1
if exist "%TEMP%\%ASSET%" del /q "%TEMP%\%ASSET%" >nul 2>&1
if exist "%TEMP%\download_url.txt" del /q "%TEMP%\download_url.txt" >nul 2>&1

REM ── Verify ────────────────────────────────────────────────────────────────────
echo.
echo   Verifying...
"%BIN_DIR%\oviec.exe" --version 2>&1 | findstr /i "ovie"
if %errorlevel% neq 0 (
    echo   [ERROR] Verification failed — oviec.exe did not run correctly.
    pause
    exit /b 1
)
echo   [OK] Verification passed

REM ── Done ──────────────────────────────────────────────────────────────────────
echo.
echo   ============================================================================
echo   ^|                  INSTALLATION COMPLETE!                                 ^|
echo   ============================================================================
echo.
echo   IMPORTANT: Restart your terminal for PATH changes to take effect.
echo.
echo   Quick start:
echo     oviec --version              ^| Check version
echo     oviec --self-check           ^| Validate installation
echo     oviec run examples\hello.ov  ^| Run hello world
echo     oviec new my-project         ^| Create new project
echo.
echo   Downloads for all platforms:
echo     Windows x64  : https://github.com/%GITHUB_REPO%/releases
echo     macOS x64    : https://github.com/%GITHUB_REPO%/releases
echo     macOS arm64  : https://github.com/%GITHUB_REPO%/releases
echo     Linux x64    : https://github.com/%GITHUB_REPO%/releases
echo     Linux arm64  : https://github.com/%GITHUB_REPO%/releases
echo.
echo   Docs: https://southwarridev.github.io/ovie/docs/book/index.html
echo.
pause
exit /b 0