@echo off
:: ============================================================================
::                    OVIE PROGRAMMING LANGUAGE v2.3.0
::                    Windows Installer - Full Package
::                    Publisher: Ovie Language Team
::                    License: MIT
:: ============================================================================

:: Request Administrator privileges
>nul 2>&1 "%SYSTEMROOT%\system32\cacls.exe" "%SYSTEMROOT%\system32\config\system"
if '%errorlevel%' NEQ '0' (
    echo Requesting Administrator privileges...
    goto UACPrompt
) else ( goto gotAdmin )

:UACPrompt
    echo Set UAC = CreateObject^("Shell.Application"^) > "%temp%\getadmin.vbs"
    echo UAC.ShellExecute "%~s0", "", "", "runas", 1 >> "%temp%\getadmin.vbs"
    "%temp%\getadmin.vbs"
    del "%temp%\getadmin.vbs"
    exit /B

:gotAdmin
    pushd "%CD%"
    CD /D "%~dp0"

cls
echo.
echo  ============================================================================
echo  ^|                                                                          ^|
echo  ^|              OVIE PROGRAMMING LANGUAGE v2.3.0                           ^|
echo  ^|              Complete Module System - Full Package                       ^|
echo  ^|              Publisher: Ovie Language Team                               ^|
echo  ^|              License: MIT Open Source                                    ^|
echo  ^|                                                                          ^|
echo  ============================================================================
echo.
echo  This installer will set up Ovie v2.3.0 with:
echo.
echo    [*] oviec.exe  - Ovie Compiler (self-hosted)
echo    [*] ovie.exe   - Ovie CLI and project manager
echo    [*] std\       - Complete standard library (11 modules)
echo    [*] std\module - v2.3 Module System
echo    [*] std\aproko - Aproko Knowledge Base
echo    [*] examples\  - 22+ runnable example programs
echo    [*] docs\      - Complete documentation
echo    [*] PATH       - Added to system PATH
echo.
echo  Installation directory: C:\Program Files\Ovie
echo.

set /p CONFIRM="Press ENTER to install or Ctrl+C to cancel: "

echo.
echo  [1/7] Creating installation directory...
if not exist "C:\Program Files\Ovie" mkdir "C:\Program Files\Ovie"
if not exist "C:\Program Files\Ovie\bin" mkdir "C:\Program Files\Ovie\bin"
if not exist "C:\Program Files\Ovie\std" mkdir "C:\Program Files\Ovie\std"
if not exist "C:\Program Files\Ovie\examples" mkdir "C:\Program Files\Ovie\examples"
if not exist "C:\Program Files\Ovie\docs" mkdir "C:\Program Files\Ovie\docs"
echo  [OK] Directories created

echo.
echo  [2/7] Installing Ovie compiler (oviec.exe)...
copy /Y "oviec.exe" "C:\Program Files\Ovie\bin\oviec.exe" >nul
if exist "C:\Program Files\Ovie\bin\oviec.exe" (
    echo  [OK] oviec.exe installed
) else (
    echo  [ERROR] Failed to install oviec.exe
    goto :error
)

echo.
echo  [3/7] Installing Ovie CLI (ovie.exe)...
copy /Y "ovie.exe" "C:\Program Files\Ovie\bin\ovie.exe" >nul 2>&1
if not exist "C:\Program Files\Ovie\bin\ovie.exe" (
    :: Create ovie.exe wrapper if not present
    copy /Y "oviec.exe" "C:\Program Files\Ovie\bin\ovie.exe" >nul
)
echo  [OK] ovie.exe installed

echo.
echo  [4/7] Installing standard library (v2.3 modules)...
xcopy /E /I /Y "std" "C:\Program Files\Ovie\std" >nul
echo  [OK] Standard library installed (std::core, std::math, std::io, std::fs,
echo       std::time, std::env, std::cli, std::log, std::testing,
echo       std::module [NEW v2.3], std::aproko [NEW v2.3])

echo.
echo  [5/7] Installing examples and documentation...
xcopy /E /I /Y "examples" "C:\Program Files\Ovie\examples" >nul
xcopy /E /I /Y "docs" "C:\Program Files\Ovie\docs" >nul
copy /Y "README.md" "C:\Program Files\Ovie\" >nul 2>&1
copy /Y "LICENSE" "C:\Program Files\Ovie\" >nul 2>&1
copy /Y "RELEASE_NOTES_v2.3.md" "C:\Program Files\Ovie\" >nul 2>&1
if exist "ovie.png" copy /Y "ovie.png" "C:\Program Files\Ovie\" >nul 2>&1
if exist "ovie.svg" copy /Y "ovie.svg" "C:\Program Files\Ovie\" >nul 2>&1
echo  [OK] Examples and documentation installed

echo.
echo  [6/7] Adding Ovie to system PATH...
:: Add to system PATH (requires admin)
for /f "tokens=2*" %%a in ('reg query "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment" /v PATH 2^>nul') do set "SYSPATH=%%b"
echo %SYSPATH% | findstr /i "Program Files\Ovie\bin" >nul
if errorlevel 1 (
    setx /M PATH "%SYSPATH%;C:\Program Files\Ovie\bin" >nul 2>&1
    echo  [OK] Added to system PATH
) else (
    echo  [OK] Already in system PATH
)

echo.
echo  [7/7] Creating Start Menu shortcut...
if not exist "%PROGRAMDATA%\Microsoft\Windows\Start Menu\Programs\Ovie" (
    mkdir "%PROGRAMDATA%\Microsoft\Windows\Start Menu\Programs\Ovie" >nul 2>&1
)
:: Create a simple shortcut via VBScript
echo Set oWS = WScript.CreateObject("WScript.Shell") > "%temp%\ovie_shortcut.vbs"
echo sLinkFile = "%PROGRAMDATA%\Microsoft\Windows\Start Menu\Programs\Ovie\Ovie Compiler.lnk" >> "%temp%\ovie_shortcut.vbs"
echo Set oLink = oWS.CreateShortcut(sLinkFile) >> "%temp%\ovie_shortcut.vbs"
echo oLink.TargetPath = "C:\Program Files\Ovie\bin\oviec.exe" >> "%temp%\ovie_shortcut.vbs"
echo oLink.WorkingDirectory = "C:\Program Files\Ovie" >> "%temp%\ovie_shortcut.vbs"
echo oLink.Description = "Ovie Programming Language Compiler v2.3.0" >> "%temp%\ovie_shortcut.vbs"
echo oLink.Save >> "%temp%\ovie_shortcut.vbs"
cscript //nologo "%temp%\ovie_shortcut.vbs" >nul 2>&1
del "%temp%\ovie_shortcut.vbs" >nul 2>&1
echo  [OK] Start Menu shortcut created

echo.
echo  ============================================================================
echo  ^|                    INSTALLATION COMPLETE!                               ^|
echo  ============================================================================
echo.
echo  Ovie v2.3.0 has been installed to: C:\Program Files\Ovie
echo.
echo  IMPORTANT: Please restart your terminal/PowerShell for PATH to take effect.
echo.
echo  Quick Start:
echo    oviec --version              Check version (2.3.0)
echo    oviec --self-check           Validate installation
echo    oviec run examples\hello.ov  Run hello world
echo    oviec new my-project         Create new project
echo.
echo  What's included in v2.3:
echo    * Complete Module System (use/import/export)
echo    * Aproko Knowledge Base for AI/LLM integration
echo    * 11 standard library modules
echo    * Self-hosted compiler
echo    * 22+ example programs
echo    * Complete documentation
echo.
echo  Resources:
echo    Website:  https://ovie-lang.org
echo    GitHub:   https://github.com/southwarridev/ovie
echo    Docs:     C:\Program Files\Ovie\docs\
echo    Examples: C:\Program Files\Ovie\examples\
echo.
echo  Thank you for installing Ovie!
echo.
pause
exit /B 0

:error
echo.
echo  [ERROR] Installation failed. Please run as Administrator.
echo  Right-click install.bat and select "Run as administrator"
echo.
pause
exit /B 1
