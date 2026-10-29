@echo off
:: oviev — Ovie self-hosted compiler shim (Stage 1)
:: Routes commands to oviec during bootstrap phase
:: Copyright (c) 2026 Nashedy Ltd — MIT License
setlocal
set OVIEV_SRC=%~dp0oviev\src
set OVIEC=%~dp0target\release\oviec.exe
if not exist "%OVIEC%" set OVIEC=oviec
if "%1" == "--version" (
    %OVIEC% run "%OVIEV_SRC%\main.ov"
    goto :eof
)
if "%1" == "--help" (
    %OVIEC% run "%OVIEV_SRC%\main.ov"
    goto :eof
)
if "%1" == "run" (
    %OVIEC% run %2 %3 %4 %5
    goto :eof
)
if "%1" == "check" (
    %OVIEC% check %2
    goto :eof
)
if "%1" == "analyze" (
    %OVIEC% analyze %2
    goto :eof
)
:: Default: pass everything to oviec
%OVIEC% %*