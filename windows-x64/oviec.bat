@echo off
setlocal

REM Get the directory where this batch file is located
set "OVIE_DIR=%~dp0"

REM Show branding information
echo Ovie Compiler (oviec) v2.2.0 - Complete Language Consolidation
echo Development Mode - Source Installation

REM Check if we're running from the correct directory
if not exist "%OVIE_DIR%oviec.exe" (
    echo Error: oviec.exe not found in %OVIE_DIR%
    echo Please ensure Ovie is properly installed.
    exit /b 1
)

REM Set up environment
set "OVIE_STD_PATH=%OVIE_DIR%std"
set "PATH=%OVIE_DIR%;%PATH%"

REM Run the actual Ovie compiler executable with all arguments
"%OVIE_DIR%oviec.exe" %*