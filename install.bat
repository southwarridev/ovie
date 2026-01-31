@echo off
REM Ovie Programming Language - Windows Command Prompt Installer
REM Stage 2: Self-Hosted Compiler Installation Script
REM This batch file installs Ovie on Windows systems via Command Prompt

setlocal enabledelayedexpansion

REM Configuration
set "OVIE_VERSION=2.0.0"
set "INSTALL_DIR=%USERPROFILE%\.local\bin"
set "TEMP_DIR=%TEMP%\ovie-install"
set "GITHUB_REPO=https://github.com/southwarridev/ovie"

echo.
echo ╔══════════════════════════════════════════════════════════════╗
echo ║                    Ovie Programming Language                 ║
echo ║                        Stage 2 - Self-Hosted                ║
echo ║                           Version %OVIE_VERSION%                     ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.
echo    ██████╗ ██╗   ██╗██╗███████╗
echo   ██╔═══██╗██║   ██║██║██╔════╝
echo   ██║   ██║██║   ██║██║█████╗  
echo   ██║   ██║╚██╗ ██╔╝██║██╔══╝  
echo   ╚██████╔╝ ╚████╔╝ ██║███████╗
echo    ╚═════╝   ╚═══╝  ╚═╝╚══════╝
echo.
echo   Natural Language Programming
echo   AI-Friendly • Memory Safe • Self-Hosted
echo.

echo [INFO] Installing Ovie Programming Language v%OVIE_VERSION%...
echo.

REM Check if we're running as administrator (optional but recommended)
net session >nul 2>&1
if %errorLevel% == 0 (
    echo [INFO] Running with administrator privileges
) else (
    echo [WARNING] Not running as administrator - installing to user directory
)

REM Create install directory
if not exist "%INSTALL_DIR%" (
    echo [INFO] Creating install directory: %INSTALL_DIR%
    mkdir "%INSTALL_DIR%" 2>nul
    if !errorlevel! neq 0 (
        echo [ERROR] Failed to create install directory
        pause
        exit /b 1
    )
)

REM Check for Git
git --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Git is required but not found in PATH
    echo [ERROR] Please install Git from: https://git-scm.com/download/win
    echo [ERROR] Or download Ovie manually from: %GITHUB_REPO%/releases
    pause
    exit /b 1
)

REM Try to download pre-built binaries first
echo [INFO] Attempting to download pre-built binaries...
set "DOWNLOAD_URL=%GITHUB_REPO%/releases/download/v%OVIE_VERSION%/ovie-v%OVIE_VERSION%-windows-x64.zip"
set "TEMP_ZIP=%TEMP%\ovie-windows-x64.zip"

REM Use PowerShell to download (available on all modern Windows)
powershell -Command "try { Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%TEMP_ZIP%' -ErrorAction Stop; exit 0 } catch { exit 1 }" >nul 2>&1

if %errorlevel% == 0 (
    echo [SUCCESS] Downloaded pre-built binaries
    
    REM Extract using PowerShell
    powershell -Command "Expand-Archive -Path '%TEMP_ZIP%' -DestinationPath '%TEMP_DIR%' -Force" >nul 2>&1
    if !errorlevel! == 0 (
        echo [INFO] Extracting binaries...
        xcopy "%TEMP_DIR%\windows-x64\*" "%INSTALL_DIR%\" /E /Y >nul 2>&1
        del "%TEMP_ZIP%" >nul 2>&1
        rmdir /s /q "%TEMP_DIR%" >nul 2>&1
        echo [SUCCESS] Pre-built binaries installed
        goto :verify_installation
    )
)

echo [WARNING] Pre-built binaries not available. Building from source...

REM Clean up temp directory
if exist "%TEMP_DIR%" rmdir /s /q "%TEMP_DIR%" >nul 2>&1

REM Clone repository
echo [INFO] Cloning Ovie repository...
git clone "%GITHUB_REPO%.git" "%TEMP_DIR%" >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Failed to clone repository
    pause
    exit /b 1
)

cd /d "%TEMP_DIR%"

REM Check if we have oviec binary for self-hosted build
if exist "oviec.exe" (
    echo [INFO] Building with self-hosted Ovie compiler...
    oviec.exe --build-all --output-dir="%INSTALL_DIR%" >nul 2>&1
    if !errorlevel! == 0 (
        echo [SUCCESS] Built with self-hosted compiler!
        goto :copy_resources
    )
)

REM Check for Rust as fallback (bootstrap only)
cargo --version >nul 2>&1
if %errorlevel% == 0 (
    echo [WARNING] Using Rust for bootstrap build (one-time only)...
    echo [INFO] Building Ovie compiler...
    cargo build --release --workspace >nul 2>&1
    if !errorlevel! neq 0 (
        echo [ERROR] Build failed. Please check your Rust installation.
        pause
        exit /b 1
    )
    
    REM Copy binaries
    copy "target\release\ovie.exe" "%INSTALL_DIR%\" >nul 2>&1
    copy "target\release\oviec.exe" "%INSTALL_DIR%\" >nul 2>&1
    echo [SUCCESS] Bootstrap build completed
) else (
    echo [ERROR] Cannot build Ovie: No self-hosted compiler or Rust toolchain found
    echo [ERROR] Please install Rust from: https://rustup.rs/
    echo [ERROR] Or download a pre-built binary from: %GITHUB_REPO%/releases
    pause
    exit /b 1
)

:copy_resources
REM Copy additional resources
if exist "ovie.png" copy "ovie.png" "%INSTALL_DIR%\" >nul 2>&1
if exist "README.md" copy "README.md" "%INSTALL_DIR%\" >nul 2>&1
if exist "LICENSE" copy "LICENSE" "%INSTALL_DIR%\" >nul 2>&1

REM Copy standard library and examples
if exist "std" xcopy "std" "%INSTALL_DIR%\std\" /E /Y >nul 2>&1
if exist "examples" xcopy "examples" "%INSTALL_DIR%\examples\" /E /Y >nul 2>&1
if exist "docs" xcopy "docs" "%INSTALL_DIR%\docs\" /E /Y >nul 2>&1

:verify_installation
REM Clean up
cd /d "%USERPROFILE%"
if exist "%TEMP_DIR%" rmdir /s /q "%TEMP_DIR%" >nul 2>&1

REM Verify installation
echo [INFO] Verifying installation...
if exist "%INSTALL_DIR%\ovie.exe" (
    if exist "%INSTALL_DIR%\oviec.exe" (
        echo [SUCCESS] Ovie Stage 2 installed successfully!
        echo.
        echo [INFO] Installation location: %INSTALL_DIR%
        echo [INFO] Binaries installed:
        echo   • ovie.exe  - CLI tool and project manager
        echo   • oviec.exe - Self-hosted Ovie compiler
        echo.
    ) else (
        echo [ERROR] oviec.exe not found in installation directory
        goto :installation_failed
    )
) else (
    echo [ERROR] ovie.exe not found in installation directory
    goto :installation_failed
)

REM Add to PATH
echo [INFO] Adding %INSTALL_DIR% to user PATH...
for /f "usebackq tokens=2,*" %%A in (`reg query HKCU\Environment /v PATH 2^>nul`) do set "CURRENT_PATH=%%B"
if not defined CURRENT_PATH set "CURRENT_PATH="

echo !CURRENT_PATH! | findstr /C:"%INSTALL_DIR%" >nul
if %errorlevel% neq 0 (
    if defined CURRENT_PATH (
        set "NEW_PATH=!CURRENT_PATH!;%INSTALL_DIR%"
    ) else (
        set "NEW_PATH=%INSTALL_DIR%"
    )
    reg add HKCU\Environment /v PATH /t REG_EXPAND_SZ /d "!NEW_PATH!" /f >nul 2>&1
    if !errorlevel! == 0 (
        echo [SUCCESS] Added to PATH. Please restart your command prompt.
    ) else (
        echo [WARNING] Failed to add to PATH automatically.
        echo [INFO] Please add %INSTALL_DIR% to your PATH manually.
    )
) else (
    echo [INFO] Directory already in PATH
)

echo.
echo ╔══════════════════════════════════════════════════════════════╗
echo ║                    Installation Complete!                   ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.
echo 🎯 Stage 2 Features Available:
echo   • Natural language programming syntax
echo   • Self-hosted compilation with oviec
echo   • LLVM backend optimization
echo   • WebAssembly compilation target
echo   • Aproko static analysis
echo   • Cross-platform deployment
echo   • Memory safety guarantees
echo   • AI-friendly development
echo.
echo 🚀 Quick Start:
echo   ovie new my-project    # Create a new project
echo   cd my-project
echo   ovie run               # Run your project
echo   ovie aproko            # Run code analysis
echo   ovie compile --wasm    # Compile to WebAssembly
echo.
echo 📚 Documentation: https://ovie-lang.org
echo 💬 Community: https://github.com/southwarridev/ovie
echo.
echo Press any key to exit...
pause >nul
exit /b 0

:installation_failed
echo [ERROR] Installation verification failed
echo [ERROR] Please check the error messages above and try again
echo [ERROR] For help, visit: https://github.com/southwarridev/ovie/issues
echo.
pause
exit /b 1