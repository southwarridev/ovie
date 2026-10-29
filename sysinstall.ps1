$ErrorActionPreference="Continue"
$log="c:\Users\Home PC\Desktop\ovie\sysinstall_log.txt"
"=== ELEVATED REPLACE $(Get-Date) ===" | Set-Content $log
try {
  $bin="C:\Program Files\Ovie\bin"
  Copy-Item "c:\Users\Home PC\Desktop\ovie\target\release\oviec.exe" "$bin\oviec.exe" -Force
  Copy-Item "c:\Users\Home PC\Desktop\ovie\target\release\oviec.exe" "$bin\ovie.exe" -Force
  "REPLACED binaries" | Add-Content $log
  $v=& "$bin\oviec.exe" --version 2>&1 | Select-String -Pattern "Stage" | Select-Object -First 1
  "VERIFY: $v" | Add-Content $log
  "DONE_OK" | Add-Content $log
} catch { "ERROR: $($_.Exception.Message)" | Add-Content $log }
