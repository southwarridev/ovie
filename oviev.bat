@echo off
:: oviev - Ovie Lang 1.0 self-hosted compiler
:: Nashedy Ltd - MIT License
setlocal
set OVIEC="%~dp0oviec.exe"
if "%1"=="run" ( %OVIEC% run %2 %3 %4 %5 & goto :eof )
if "%1"=="check" ( %OVIEC% check %2 & goto :eof )
if "%1"=="analyze" ( %OVIEC% analyze %2 & goto :eof )
if "%1"=="compile" ( %OVIEC% compile %2 %3 %4 %5 & goto :eof )
if "%1"=="--version" ( %OVIEC% run "%~dp0..\oviev\src\main.ov" & goto :eof )
if "%1"=="--help" ( %OVIEC% run "%~dp0..\oviev\src\main.ov" & goto :eof )
if "%1"=="" ( %OVIEC% run "%~dp0..\oviev\src\main.ov" ) else ( %OVIEC% run %1 %2 %3 )
