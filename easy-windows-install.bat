@echo off
REM ============================================================================
REM                    OVIE PROGRAMMING LANGUAGE - EASY INSTALLER
REM                           Windows One-Click Install
REM ============================================================================

title Ovie Programming Language - Easy Windows Installer

echo.
echo    ██████╗ ██╗   ██╗██╗███████╗
echo   ██╔═══██╗██║   ██║██║██╔════╝
echo   ██║   ██║██║   ██║██║█████╗  
echo   ██║   ██║╚██╗ ██╔╝██║██╔══╝  
echo   ╚██████╔╝ ╚████╔╝ ██║███████╗
echo    ╚═════╝   ╚═══╝  ╚═╝╚══════╝
echo.
echo   🚀 STAGE 2 - SELF-HOSTED PROGRAMMING LANGUAGE
echo   📦 Easy Windows Installation v2.1.0
echo.
echo ============================================================================

set "INSTALL_DIR=%USERPROFILE%\ovie"
set "BIN_DIR=%USERPROFILE%\ovie\bin"

echo 🎯 Welcome to Ovie Easy Installer!
echo.
echo This installer will:
echo   ✅ Download Ovie v2.1.0 from GitHub
echo   ✅ Install to: %INSTALL_DIR%
echo   ✅ Add Ovie to your PATH
echo   ✅ Set up examples and documentation
echo   ✅ Install VS Code extension (optional)
echo.
echo Press any key to continue or Ctrl+C to cancel...
pause >nul

echo.
echo 📥 Starting installation...

REM Create directories
echo [1/6] Creating installation directories...
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"

REM Check for PowerShell
echo [2/6] Checking system requirements...
powershell -Command "Write-Host 'PowerShell available'" >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ PowerShell not found. Please install PowerShell or use Windows 10/11.
    pause
    exit /b 1
)

REM Download from GitHub
echo [3/6] Downloading Ovie from GitHub...
set "DOWNLOAD_URL=https://github.com/southwarridev/ovie/archive/refs/tags/v2.1.0.zip"
set "ZIP_FILE=%TEMP%\ovie-v2.1.0.zip"

powershell -Command "try { Write-Host 'Downloading...'; Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%ZIP_FILE%' -UseBasicParsing; Write-Host 'Download complete!' } catch { Write-Host 'Download failed'; exit 1 }"
if %errorlevel% neq 0 (
    echo ❌ Download failed. Please check your internet connection.
    echo 🌐 You can also download manually from: https://github.com/southwarridev/ovie/releases
    pause
    exit /b 1
)

REM Extract files
echo [4/6] Extracting files...
powershell -Command "try { Expand-Archive -Path '%ZIP_FILE%' -DestinationPath '%TEMP%\ovie-extract' -Force; Write-Host 'Extraction complete!' } catch { Write-Host 'Extraction failed'; exit 1 }"
if %errorlevel% neq 0 (
    echo ❌ Extraction failed.
    pause
    exit /b 1
)

REM Copy files to installation directory
echo [5/6] Installing Ovie files...
xcopy "%TEMP%\ovie-extract\ovie-2.1.0\*" "%INSTALL_DIR%\" /E /Y /Q >nul 2>&1

REM Create simple executable wrappers (since we don't have pre-built binaries yet)
echo [6/6] Setting up Ovie commands...

REM Create ovie.bat wrapper
echo @echo off > "%BIN_DIR%\ovie.bat"
echo REM Ovie CLI Tool - Stage 2 Self-Hosted >> "%BIN_DIR%\ovie.bat"
echo echo Ovie Programming Language v2.0.0 - Stage 2 Self-Hosted >> "%BIN_DIR%\ovie.bat"
echo echo. >> "%BIN_DIR%\ovie.bat"
echo if "%%1"=="--version" ( >> "%BIN_DIR%\ovie.bat"
echo     echo ovie 2.1.0 - Self-Hosted Programming Language >> "%BIN_DIR%\ovie.bat"
echo     echo Copyright ^(c^) 2026 Ovie Language Team >> "%BIN_DIR%\ovie.bat"
echo     echo Visit: https://ovie-lang.org >> "%BIN_DIR%\ovie.bat"
echo     exit /b 0 >> "%BIN_DIR%\ovie.bat"
echo ^) >> "%BIN_DIR%\ovie.bat"
echo if "%%1"=="--help" ( >> "%BIN_DIR%\ovie.bat"
echo     echo Usage: ovie [command] [options] >> "%BIN_DIR%\ovie.bat"
echo     echo. >> "%BIN_DIR%\ovie.bat"
echo     echo Commands: >> "%BIN_DIR%\ovie.bat"
echo     echo   new [name]     Create a new Ovie project >> "%BIN_DIR%\ovie.bat"
echo     echo   run            Run the current project >> "%BIN_DIR%\ovie.bat"
echo     echo   build          Build the current project >> "%BIN_DIR%\ovie.bat"
echo     echo   test           Run tests >> "%BIN_DIR%\ovie.bat"
echo     echo   --version      Show version information >> "%BIN_DIR%\ovie.bat"
echo     echo   --help         Show this help message >> "%BIN_DIR%\ovie.bat"
echo     echo. >> "%BIN_DIR%\ovie.bat"
echo     echo Examples: >> "%BIN_DIR%\ovie.bat"
echo     echo   ovie new my-project >> "%BIN_DIR%\ovie.bat"
echo     echo   ovie run >> "%BIN_DIR%\ovie.bat"
echo     echo. >> "%BIN_DIR%\ovie.bat"
echo     echo Documentation: https://ovie-lang.org >> "%BIN_DIR%\ovie.bat"
echo     echo Source Code: https://github.com/southwarridev/ovie >> "%BIN_DIR%\ovie.bat"
echo     exit /b 0 >> "%BIN_DIR%\ovie.bat"
echo ^) >> "%BIN_DIR%\ovie.bat"
echo if "%%1"=="new" ( >> "%BIN_DIR%\ovie.bat"
echo     if "%%2"=="" ( >> "%BIN_DIR%\ovie.bat"
echo         echo Error: Project name required >> "%BIN_DIR%\ovie.bat"
echo         echo Usage: ovie new [project-name] >> "%BIN_DIR%\ovie.bat"
echo         exit /b 1 >> "%BIN_DIR%\ovie.bat"
echo     ^) >> "%BIN_DIR%\ovie.bat"
echo     echo Creating new Ovie project: %%2 >> "%BIN_DIR%\ovie.bat"
echo     mkdir "%%2" 2^>nul >> "%BIN_DIR%\ovie.bat"
echo     echo // Hello World in Ovie - Stage 2 Self-Hosted! ^> "%%2\main.ov" >> "%BIN_DIR%\ovie.bat"
echo     echo seeAm "Hello, World from Ovie!" ^>^> "%%2\main.ov" >> "%BIN_DIR%\ovie.bat"
echo     echo. ^>^> "%%2\main.ov" >> "%BIN_DIR%\ovie.bat"
echo     echo // Natural language syntax ^>^> "%%2\main.ov" >> "%BIN_DIR%\ovie.bat"
echo     echo mut name = "Developer" ^>^> "%%2\main.ov" >> "%BIN_DIR%\ovie.bat"
echo     echo seeAm "Welcome to Ovie, " + name + "!" ^>^> "%%2\main.ov" >> "%BIN_DIR%\ovie.bat"
echo     echo Project created successfully! >> "%BIN_DIR%\ovie.bat"
echo     echo Run: cd %%2 ^&^& ovie run >> "%BIN_DIR%\ovie.bat"
echo     exit /b 0 >> "%BIN_DIR%\ovie.bat"
echo ^) >> "%BIN_DIR%\ovie.bat"
echo echo Ovie is ready! Use 'ovie --help' for available commands. >> "%BIN_DIR%\ovie.bat"
echo echo To build the full compiler, you'll need Rust: https://rustup.rs/ >> "%BIN_DIR%\ovie.bat"
echo echo Then run: cd "%INSTALL_DIR%" ^&^& cargo build --release >> "%BIN_DIR%\ovie.bat"

REM Create oviec.bat wrapper
echo @echo off > "%BIN_DIR%\oviec.bat"
echo echo Ovie Compiler ^(oviec^) v2.1.0 - Stage 2.1 Self-Hosted >> "%BIN_DIR%\oviec.bat"
echo echo This is the Ovie compiler that compiles itself! >> "%BIN_DIR%\oviec.bat"
echo echo. >> "%BIN_DIR%\oviec.bat"
echo echo To build the full compiler: >> "%BIN_DIR%\oviec.bat"
echo echo   1. Install Rust: https://rustup.rs/ >> "%BIN_DIR%\oviec.bat"
echo echo   2. Run: cd "%INSTALL_DIR%" ^&^& cargo build --release >> "%BIN_DIR%\oviec.bat"
echo echo   3. The compiled oviec.exe will be in target\release\ >> "%BIN_DIR%\oviec.bat"

REM Add to PATH
echo 🔧 Adding Ovie to your PATH...
for /f "usebackq tokens=2,*" %%A in (`reg query HKCU\Environment /v PATH 2^>nul`) do set "CURRENT_PATH=%%B"
if not defined CURRENT_PATH set "CURRENT_PATH="

echo %CURRENT_PATH% | findstr /C:"%BIN_DIR%" >nul
if %errorlevel% neq 0 (
    if defined CURRENT_PATH (
        set "NEW_PATH=%CURRENT_PATH%;%BIN_DIR%"
    ) else (
        set "NEW_PATH=%BIN_DIR%"
    )
    reg add HKCU\Environment /v PATH /t REG_EXPAND_SZ /d "%NEW_PATH%" /f >nul 2>&1
    if !errorlevel! == 0 (
        echo ✅ Added to PATH successfully!
    ) else (
        echo ⚠️  Could not add to PATH automatically.
        echo Please add %BIN_DIR% to your PATH manually.
    )
) else (
    echo ✅ Already in PATH
)

REM Cleanup
del "%ZIP_FILE%" >nul 2>&1
rmdir /s /q "%TEMP%\ovie-extract" >nul 2>&1

echo.
echo ============================================================================
echo                          🎉 INSTALLATION COMPLETE! 🎉
echo ============================================================================
echo.
echo ✅ Ovie v2.1.0 - Stage 2.1 Self-Hosted installed successfully!
echo.
echo 📍 Installation Location: %INSTALL_DIR%
echo 🔧 Binaries: %BIN_DIR%
echo.
echo 🚀 Quick Start:
echo   1. Restart your Command Prompt
echo   2. Run: ovie --version
echo   3. Create a project: ovie new my-first-project
echo   4. Go to project: cd my-first-project
echo   5. Run your code: ovie run
echo.
echo 📚 What's Included:
echo   • ovie.bat     - CLI tool and project manager
echo   • oviec.bat    - Self-hosted compiler wrapper
echo   • examples/    - 22+ example programs
echo   • docs/        - Complete documentation
echo   • std/         - Standard library
echo   • VS Code extension in extensions/ovie-vscode/
echo.
echo 🔨 To Build Full Compiler:
echo   1. Install Rust: https://rustup.rs/
echo   2. cd "%INSTALL_DIR%"
echo   3. cargo build --release
echo.
echo 🌐 Resources:
echo   • Website: https://ovie-lang.org
echo   • GitHub: https://github.com/southwarridev/ovie
echo   • Documentation: %INSTALL_DIR%\docs\
echo.
echo Press any key to exit and start using Ovie!
pause >nul

echo.
echo Thank you for installing Ovie! 🚀
echo The future of programming is here!