@echo off
REM Simple Ovie Windows Installer Helper
REM For when you have the source code already downloaded

echo.
echo ╔══════════════════════════════════════════════════════════════╗
echo ║                    Ovie Programming Language                 ║
echo ║                    Windows Installation Helper               ║
echo ║                        Version 2.0.0                        ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.

set "INSTALL_DIR=%USERPROFILE%\.local\bin"
set "CURRENT_DIR=%CD%"

echo [INFO] Current directory: %CURRENT_DIR%
echo [INFO] Install directory: %INSTALL_DIR%
echo.

REM Create install directory
if not exist "%INSTALL_DIR%" (
    echo [INFO] Creating install directory...
    mkdir "%INSTALL_DIR%"
)

REM Check if we have Rust installed
echo [INFO] Checking for Rust toolchain...
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust is not installed or not in PATH
    echo [INFO] Installing Rust...
    echo [INFO] Downloading Rust installer...
    
    REM Download and run Rust installer
    powershell -Command "Invoke-WebRequest -Uri 'https://win.rustup.rs/x86_64' -OutFile '%TEMP%\rustup-init.exe'"
    if exist "%TEMP%\rustup-init.exe" (
        echo [INFO] Running Rust installer...
        "%TEMP%\rustup-init.exe" -y
        call "%USERPROFILE%\.cargo\env.bat"
        del "%TEMP%\rustup-init.exe"
    ) else (
        echo [ERROR] Failed to download Rust installer
        echo [INFO] Please install Rust manually from: https://rustup.rs/
        pause
        exit /b 1
    )
)

echo [SUCCESS] Rust toolchain found
echo.

REM Build Ovie
echo [INFO] Building Ovie from source...
cargo build --release --workspace
if %errorlevel% neq 0 (
    echo [ERROR] Build failed
    pause
    exit /b 1
)

echo [SUCCESS] Build completed
echo.

REM Copy binaries
echo [INFO] Installing binaries...
if exist "target\release\ovie.exe" (
    copy "target\release\ovie.exe" "%INSTALL_DIR%\" >nul
    echo [SUCCESS] Copied ovie.exe
) else (
    echo [ERROR] ovie.exe not found in target\release\
)

if exist "target\release\oviec.exe" (
    copy "target\release\oviec.exe" "%INSTALL_DIR%\" >nul
    echo [SUCCESS] Copied oviec.exe
) else (
    echo [ERROR] oviec.exe not found in target\release\
)

REM Copy resources
if exist "ovie.png" copy "ovie.png" "%INSTALL_DIR%\" >nul
if exist "README.md" copy "README.md" "%INSTALL_DIR%\" >nul
if exist "LICENSE" copy "LICENSE" "%INSTALL_DIR%\" >nul

REM Copy directories
if exist "std" xcopy "std" "%INSTALL_DIR%\std\" /E /Y >nul
if exist "examples" xcopy "examples" "%INSTALL_DIR%\examples\" /E /Y >nul
if exist "docs" xcopy "docs" "%INSTALL_DIR%\docs\" /E /Y >nul

echo.
echo [INFO] Adding to PATH...
REM Add to PATH
for /f "usebackq tokens=2,*" %%A in (`reg query HKCU\Environment /v PATH 2^>nul`) do set "CURRENT_PATH=%%B"
if not defined CURRENT_PATH set "CURRENT_PATH="

echo %CURRENT_PATH% | findstr /C:"%INSTALL_DIR%" >nul
if %errorlevel% neq 0 (
    if defined CURRENT_PATH (
        set "NEW_PATH=%CURRENT_PATH%;%INSTALL_DIR%"
    ) else (
        set "NEW_PATH=%INSTALL_DIR%"
    )
    reg add HKCU\Environment /v PATH /t REG_EXPAND_SZ /d "%NEW_PATH%" /f >nul
    echo [SUCCESS] Added to PATH
) else (
    echo [INFO] Already in PATH
)

echo.
echo ╔══════════════════════════════════════════════════════════════╗
echo ║                    Installation Complete!                   ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.
echo 🎉 Ovie Stage 2 - Self-Hosted Programming Language installed!
echo.
echo 📍 Installation location: %INSTALL_DIR%
echo 🔧 Binaries installed:
echo    • ovie.exe  - CLI tool and project manager
echo    • oviec.exe - Self-hosted Ovie compiler
echo.
echo 🚀 Quick Start (restart your command prompt first):
echo    ovie --version
echo    ovie new hello-world
echo    cd hello-world
echo    ovie run
echo.
echo 📚 Documentation: https://ovie-lang.org
echo 💬 Community: https://github.com/southwarridev/ovie
echo.
echo Please restart your command prompt to use the 'ovie' command.
echo.
pause