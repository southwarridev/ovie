@echo off
:: oviev - Ovie self-hosted compiler (Stage 1 bootstrap)
:: Copyright (c) 2026 Nashedy Ltd - MIT License
setlocal

:: Find oviec.exe
set OVIEC=oviec
if exist "%~dp0oviec.exe" set OVIEC="%~dp0oviec.exe"

:: Route run/check/analyze/compile directly to oviec
if "%1"=="run" (
    %OVIEC% run %2 %3 %4 %5
    goto :eof
)
if "%1"=="check" (
    %OVIEC% check %2
    goto :eof
)
if "%1"=="analyze" (
    %OVIEC% analyze %2
    goto :eof
)
if "%1"=="compile" (
    %OVIEC% compile %2 %3 %4 %5
    goto :eof
)
if "%1"=="aproko" (
    %OVIEC% aproko %2 %3 %4
    goto :eof
)

:: --version and --help show oviev main.ov
if "%1"=="--version" (
    %OVIEC% run "%~dp0..\oviev\src\main.ov"
    goto :eof
)
if "%1"=="--help" (
    %OVIEC% run "%~dp0..\oviev\src\main.ov"
    goto :eof
)

:: Default: if no args, show help. If file given, run it.
if "%1"=="" (
    %OVIEC% run "%~dp0..\oviev\src\main.ov"
) else (
    %OVIEC% run %1 %2 %3
)