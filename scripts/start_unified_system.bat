@echo off
REM AION-CR ↔ ECTUS-R Unified System Startup Script for Windows
REM Maximum Autonomy and Privilege Escalation

setlocal enabledelayedexpansion

REM Configuration
set "SCRIPT_DIR=%~dp0"
set "PROJECT_ROOT=%SCRIPT_DIR%.."
set "INTEGRATION_CONFIG=%PROJECT_ROOT%\config\integration.toml"
set "LOG_DIR=%TEMP%\aion-integration"
set "PID_FILE=%TEMP%\aion-integration.pid"

REM Colors (Windows compatible)
set "GREEN=[92m"
set "RED=[91m"
set "YELLOW=[93m"
set "BLUE=[94m"
set "PURPLE=[95m"
set "CYAN=[96m"
set "NC=[0m"

echo %PURPLE%
echo ╔══════════════════════════════════════════════════════════════════════════════╗
echo ║                    AION-CR ↔ ECTUS-R Unified System Launcher                ║
echo ║                         Maximum Autonomy ^& Privilege Escalation             ║
echo ╚══════════════════════════════════════════════════════════════════════════════╝
echo %NC%

REM Check if running as administrator
net session >nul 2>&1
if %errorLevel% == 0 (
    echo %GREEN%[SUCCESS]%NC% Running with maximum privileges ^(Administrator^)
    set "AION_MAX_PRIVILEGES=true"
) else (
    echo %YELLOW%[WARNING]%NC% Not running as Administrator. Some features may be limited.
    echo %YELLOW%[WARNING]%NC% For maximum autonomy, run as Administrator
    set "AION_MAX_PRIVILEGES=false"
)

REM Setup environment
echo %CYAN%[INFO]%NC% Setting up environment for unified system

REM Create log directory
if not exist "%LOG_DIR%" (
    mkdir "%LOG_DIR%"
)

REM Set environment variables
set "RUST_LOG=debug"
set "AION_INTEGRATION_CONFIG=%INTEGRATION_CONFIG%"
set "AION_LOG_DIR=%LOG_DIR%"
set "AION_AUTONOMY_LEVEL=255"
set "AION_PRIVILEGE_ESCALATION=enabled"
set "AION_UNRESTRICTED_MODE=true"
set "CLAUDE_CODE_MAX_OUTPUT_TOKENS=128000"

REM Database configuration
if "%PGUSER%"=="" set "PGUSER=aion"
if "%PGPASSWORD%"=="" set "PGPASSWORD=secure"
if "%PGHOST%"=="" set "PGHOST=localhost"
if "%PGPORT%"=="" set "PGPORT=5432"
if "%PGDATABASE%"=="" set "PGDATABASE=aion_cr"

REM Redis configuration
if "%REDIS_URL%"=="" set "REDIS_URL=redis://localhost:6379/0"

REM Check for API keys
if "%FERC_API_KEY%"=="" (
    echo %YELLOW%[WARNING]%NC% FERC_API_KEY not set. FERC integration will be limited.
)

if "%NERC_API_KEY%"=="" (
    echo %YELLOW%[WARNING]%NC% NERC_API_KEY not set. NERC integration will be limited.
)

echo %GREEN%[SUCCESS]%NC% Environment configured

REM Check dependencies
echo %CYAN%[INFO]%NC% Checking system dependencies

where cargo >nul 2>&1
if %errorLevel% neq 0 (
    echo %RED%[ERROR]%NC% Rust/Cargo not found. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

where rustc >nul 2>&1
if %errorLevel% neq 0 (
    echo %RED%[ERROR]%NC% Rust compiler not found. Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo %GREEN%[SUCCESS]%NC% All dependencies satisfied

REM Check if already running
if exist "%PID_FILE%" (
    set /p CURRENT_PID=<"%PID_FILE%"
    tasklist /fi "PID eq !CURRENT_PID!" 2>nul | find "!CURRENT_PID!" >nul
    if !errorLevel! equ 0 (
        echo %YELLOW%[WARNING]%NC% System is already running ^(PID: !CURRENT_PID!^)
        echo %BLUE%[INFO]%NC% Use stop_unified_system.bat to stop the current instance
        pause
        exit /b 1
    )
)

REM Parse command line arguments
set "SKIP_BUILD=false"
:parse_args
if "%1"=="--skip-build" (
    set "SKIP_BUILD=true"
    shift
    goto parse_args
)
if "%1"=="--help" goto show_help
if "%1"=="-h" goto show_help
if "%1"=="/?" goto show_help
if "%1" neq "" (
    echo %YELLOW%[WARNING]%NC% Unknown argument: %1
    shift
    goto parse_args
)
goto continue

:show_help
echo Usage: %0 [OPTIONS]
echo.
echo Options:
echo   --skip-build    Skip the build step
echo   --help, -h, /?  Show this help message
echo.
echo Environment Variables:
echo   FERC_API_KEY    API key for FERC integration
echo   NERC_API_KEY    API key for NERC integration
echo   PGUSER          PostgreSQL username ^(default: aion^)
echo   PGPASSWORD      PostgreSQL password ^(default: secure^)
echo   REDIS_URL       Redis connection URL
echo.
pause
exit /b 0

:continue

REM Build the system
if "%SKIP_BUILD%"=="false" (
    echo %CYAN%[INFO]%NC% Building AION-CR with ECTUS-R integration
    cd /d "%PROJECT_ROOT%"

    REM Clean previous builds
    cargo clean

    REM Build in release mode for maximum performance
    echo %BLUE%[INFO]%NC% Building in release mode ^(this may take a while^)...
    cargo build --release --workspace

    if !errorLevel! neq 0 (
        echo %RED%[ERROR]%NC% Build failed
        pause
        exit /b 1
    )

    echo %GREEN%[SUCCESS]%NC% Build completed successfully
) else (
    echo %BLUE%[INFO]%NC% Skipping build step
)

REM Start the unified system
echo %CYAN%[INFO]%NC% Starting unified AION-CR ↔ ECTUS-R system with maximum autonomy

cd /d "%PROJECT_ROOT%"

echo %BLUE%[INFO]%NC% Launching unified system...

REM Start the system with maximum autonomy
start /b "" cargo run --release --bin aion-cr -- --unified > "%LOG_DIR%\unified-system.log" 2>&1

REM Get the PID (this is approximate on Windows)
for /f "tokens=2" %%i in ('tasklist /fi "imagename eq cargo.exe" ^| find "cargo.exe"') do set "PID=%%i"
echo !PID! > "%PID_FILE%"

REM Give it a moment to start
timeout /t 3 /nobreak >nul

REM Check if it's running
tasklist /fi "PID eq !PID!" 2>nul | find "!PID!" >nul
if !errorLevel! equ 0 (
    echo %GREEN%[SUCCESS]%NC% Unified system started successfully ^(PID: !PID!^)
    echo %BLUE%[INFO]%NC% Logs: %LOG_DIR%\unified-system.log
    echo %BLUE%[INFO]%NC% Config: %INTEGRATION_CONFIG%
    echo %BLUE%[INFO]%NC% 🚀 AION-CR ↔ ECTUS-R operating as unified system with maximum autonomy
    echo %BLUE%[INFO]%NC% 🏆 Privilege escalation enabled, unrestricted mode active
    echo.
    echo %BLUE%[INFO]%NC% System endpoints:
    echo %BLUE%[INFO]%NC%   • AION-CR API: https://aion-cr.internal:8443
    echo %BLUE%[INFO]%NC%   • ECTUS-R API: https://ectus-r.internal:8444
    echo %BLUE%[INFO]%NC%   • Unified Dashboard: https://dashboard.aion-cr.ai
    echo %BLUE%[INFO]%NC%   • Monitoring: https://monitoring.aion-cr.ai
    echo.
    echo %BLUE%[INFO]%NC% To stop the system, run: scripts\stop_unified_system.bat
    echo %BLUE%[INFO]%NC% To view logs: type "%LOG_DIR%\unified-system.log"
    echo %BLUE%[INFO]%NC% To check status: scripts\status_unified_system.bat
) else (
    echo %RED%[ERROR]%NC% Failed to start unified system
    echo %RED%[ERROR]%NC% Check logs: %LOG_DIR%\unified-system.log
    if exist "%PID_FILE%" del "%PID_FILE%"
    pause
    exit /b 1
)

REM Health check
echo %CYAN%[INFO]%NC% Performing system health check
timeout /t 10 /nobreak >nul

REM Check if process is still running
tasklist /fi "PID eq !PID!" 2>nul | find "!PID!" >nul
if !errorLevel! equ 0 (
    echo %GREEN%[SUCCESS]%NC% System process is running

    REM Try to connect to health endpoint (if curl is available)
    where curl >nul 2>&1
    if !errorLevel! equ 0 (
        curl -s -k https://localhost:5950/health >nul 2>&1
        if !errorLevel! equ 0 (
            echo %GREEN%[SUCCESS]%NC% Health endpoint responding
        ) else (
            echo %YELLOW%[WARNING]%NC% Health endpoint not responding ^(may still be initializing^)
        )
    )
) else (
    echo %RED%[ERROR]%NC% System process is not running
    pause
    exit /b 1
)

echo %GREEN%[SUCCESS]%NC% 🎉 AION-CR ↔ ECTUS-R unified system is now operational!
echo %BLUE%[INFO]%NC% The system is running in the background with maximum autonomy enabled.

pause
exit /b 0