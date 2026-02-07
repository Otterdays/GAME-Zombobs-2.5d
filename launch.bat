@echo off
title ZOMBS-ENGINE Launcher
echo ==========================================
echo       ZOMBS-ENGINE DEVELOPMENT SERVER
echo ==========================================
echo.

:: Check for Python
python --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Python is not installed or not in your PATH.
    echo Please install Python to run the local server.
    echo.
    pause
    exit /b 1
)

:: Build WASM
echo [INFO] Building WASM...
wasm-pack build --target web
if %errorlevel% neq 0 (
    echo [ERROR] WASM build failed.
    pause
    exit /b 1
)

:: Open Browser
echo [INFO] Opening default browser to http://localhost:8675...
start http://localhost:8675

:: Start Server
echo [INFO] Starting Python HTTP Server on port 8675...
echo [INFO] Press Ctrl+C to stop the server.
echo.
python -m http.server 8675
