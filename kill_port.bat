@echo off
set PORT=8675
echo Killing any process on port %PORT%...
for /f "tokens=5" %%a in ('netstat -aon ^| findstr :%PORT%') do taskkill /f /pid %%a
echo Done.
