@echo off
echo.
echo ╔══════════════════════════════════════════════════════════════╗
echo ║                    Ovie Programming Language                 ║
echo ║                        Stage 2 - Self-Hosted                ║
echo ║                           Version 2.0.0                     ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.
echo Installing Ovie Programming Language for Windows...
if not exist "%USERPROFILE%\.local\bin" mkdir "%USERPROFILE%\.local\bin"
copy ovie.bat "%USERPROFILE%\.local\bin\ovie.bat" >nul 2>&1
copy oviec.bat "%USERPROFILE%\.local\bin\oviec.bat" >nul 2>&1
echo ✓ Ovie installed successfully!
echo.
echo Add %USERPROFILE%\.local\bin to your PATH if not already added.
echo Run 'ovie --help' to get started.
echo.
echo Visit: https://ovie-lang.org for documentation
pause