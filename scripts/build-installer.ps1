# AION-CR Cross-Platform Installer Builder
# Builds self-contained executable installers for Windows, macOS, and Linux

param(
    [Parameter(Mandatory=$false)]
    [string]$Version = "1.0.0",

    [Parameter(Mandatory=$false)]
    [string]$Target = "all",

    [Parameter(Mandatory=$false)]
    [switch]$Release = $false,

    [Parameter(Mandatory=$false)]
    [switch]$Sign = $false,

    [Parameter(Mandatory=$false)]
    [switch]$Upload = $false
)

# Configuration
$ProjectRoot = Split-Path -Parent $PSScriptRoot
$BuildDir = Join-Path $ProjectRoot "build"
$InstallerDir = Join-Path $ProjectRoot "installers"
$ArtifactsDir = Join-Path $ProjectRoot "artifacts"

# Ensure directories exist
@($BuildDir, $InstallerDir, $ArtifactsDir) | ForEach-Object {
    if (-not (Test-Path $_)) {
        New-Item -ItemType Directory -Path $_ -Force | Out-Null
    }
}

Write-Host "🚀 AION-CR Installer Builder v$Version" -ForegroundColor Blue
Write-Host "Target: $Target" -ForegroundColor Green
Write-Host "Release Mode: $Release" -ForegroundColor Green
Write-Host "Code Signing: $Sign" -ForegroundColor Green

# Build targets configuration
$BuildTargets = @{
    "windows-x64" = @{
        Platform = "windows"
        Architecture = "x64"
        Extension = ".exe"
        RustTarget = "x86_64-pc-windows-msvc"
        Installer = "nsis"
    }
    "windows-arm64" = @{
        Platform = "windows"
        Architecture = "arm64"
        Extension = ".exe"
        RustTarget = "aarch64-pc-windows-msvc"
        Installer = "nsis"
    }
    "macos-x64" = @{
        Platform = "macos"
        Architecture = "x64"
        Extension = ""
        RustTarget = "x86_64-apple-darwin"
        Installer = "pkgbuild"
    }
    "macos-arm64" = @{
        Platform = "macos"
        Architecture = "arm64"
        Extension = ""
        RustTarget = "aarch64-apple-darwin"
        Installer = "pkgbuild"
    }
    "linux-x64" = @{
        Platform = "linux"
        Architecture = "x64"
        Extension = ""
        RustTarget = "x86_64-unknown-linux-gnu"
        Installer = "deb"
    }
    "linux-arm64" = @{
        Platform = "linux"
        Architecture = "arm64"
        Extension = ""
        RustTarget = "aarch64-unknown-linux-gnu"
        Installer = "deb"
    }
}

function Build-RustBinaries {
    param([hashtable]$Targets)

    Write-Host "🔨 Building Rust binaries..." -ForegroundColor Yellow

    # Install required targets
    foreach ($targetConfig in $Targets.Values) {
        Write-Host "Installing Rust target: $($targetConfig.RustTarget)" -ForegroundColor Cyan
        cargo install --target $targetConfig.RustTarget 2>$null
    }

    # Build binaries
    foreach ($targetName in $Targets.Keys) {
        $targetConfig = $Targets[$targetName]
        Write-Host "Building $targetName..." -ForegroundColor Cyan

        $buildMode = if ($Release) { "--release" } else { "" }
        $targetDir = if ($Release) { "release" } else { "debug" }

        # Build main server
        $env:CARGO_TARGET_DIR = Join-Path $BuildDir $targetName
        cargo build $buildMode --target $targetConfig.RustTarget --bin aion-server

        # Build CLI
        cargo build $buildMode --target $targetConfig.RustTarget --bin aion-cli

        # Copy binaries to build directory
        $binaryDir = Join-Path $BuildDir "$targetName\binaries"
        New-Item -ItemType Directory -Path $binaryDir -Force | Out-Null

        $sourceDir = Join-Path $env:CARGO_TARGET_DIR "$($targetConfig.RustTarget)\$targetDir"
        Copy-Item "$sourceDir\aion-server$($targetConfig.Extension)" $binaryDir -Force
        Copy-Item "$sourceDir\aion-cli$($targetConfig.Extension)" $binaryDir -Force

        Write-Host "✅ Built $targetName binaries" -ForegroundColor Green
    }
}

function Build-WebUI {
    Write-Host "🌐 Building Web UI..." -ForegroundColor Yellow

    Push-Location (Join-Path $ProjectRoot "web-ui")
    try {
        # Install dependencies
        npm ci --silent

        # Build for production
        $env:NODE_ENV = "production"
        $env:VITE_API_BASE_URL = "/api"
        $env:VITE_VERSION = $Version
        npm run build

        # Copy build output
        $webUIBuildDir = Join-Path $BuildDir "web-ui"
        if (Test-Path $webUIBuildDir) {
            Remove-Item $webUIBuildDir -Recurse -Force
        }
        Copy-Item "dist" $webUIBuildDir -Recurse -Force

        Write-Host "✅ Built Web UI" -ForegroundColor Green
    }
    finally {
        Pop-Location
    }
}

function Create-ConfigFiles {
    Write-Host "⚙️ Creating configuration files..." -ForegroundColor Yellow

    $configDir = Join-Path $BuildDir "config"
    New-Item -ItemType Directory -Path $configDir -Force | Out-Null

    # Production configuration
    $prodConfig = @"
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
url = "postgresql://aion:aion@localhost:5432/aion_cr"
max_connections = 100
ssl_mode = "prefer"

[redis]
url = "redis://localhost:6379"
pool_size = 10

[ml]
models_path = "./models"
inference_workers = 2
gpu_enabled = true

[monitoring]
metrics_enabled = true
tracing_enabled = true
log_level = "info"

[security]
jwt_secret = "change_this_in_production"
session_timeout = 3600
cors_origins = ["*"]

[compliance]
auto_update = true
real_time_monitoring = true
alert_threshold = 0.8

[autonomous]
enabled = true
max_privileges = true
auto_escalation = true
unrestricted_mode = true
"@

    $prodConfig | Out-File -FilePath (Join-Path $configDir "production.toml") -Encoding UTF8

    # Development configuration
    $devConfig = @"
[server]
host = "127.0.0.1"
port = 8080
workers = 2

[database]
url = "postgresql://aion:aion@localhost:5432/aion_cr_dev"
max_connections = 10
ssl_mode = "disable"

[redis]
url = "redis://localhost:6379"
pool_size = 5

[ml]
models_path = "./models"
inference_workers = 1
gpu_enabled = false

[monitoring]
metrics_enabled = true
tracing_enabled = true
log_level = "debug"

[security]
jwt_secret = "dev_secret_key"
session_timeout = 86400
cors_origins = ["http://localhost:3000", "http://localhost:5173"]

[compliance]
auto_update = false
real_time_monitoring = false
alert_threshold = 0.6

[autonomous]
enabled = false
max_privileges = false
auto_escalation = false
unrestricted_mode = false
"@

    $devConfig | Out-File -FilePath (Join-Path $configDir "development.toml") -Encoding UTF8

    Write-Host "✅ Created configuration files" -ForegroundColor Green
}

function Create-ServiceFiles {
    Write-Host "🔧 Creating service files..." -ForegroundColor Yellow

    $serviceDir = Join-Path $BuildDir "services"
    New-Item -ItemType Directory -Path $serviceDir -Force | Out-Null

    # Windows service definition
    $windowsService = @"
<?xml version="1.0" encoding="UTF-8"?>
<service>
    <id>aion-cr</id>
    <name>AION-CR Regulatory Compliance System</name>
    <description>Advanced AI-powered regulatory compliance management system</description>
    <executable>aion-server.exe</executable>
    <arguments>--config production.toml</arguments>
    <logmode>rotate</logmode>
    <depend>PostgreSQL</depend>
    <depend>Redis</depend>
    <startmode>Automatic</startmode>
    <delayedAutoStart>true</delayedAutoStart>
    <stopparentprocessfirst>true</stopparentprocessfirst>
    <stoptimeout>30 sec</stoptimeout>
    <env name="AION_ENV" value="production"/>
    <env name="RUST_LOG" value="info"/>
</service>
"@

    $windowsService | Out-File -FilePath (Join-Path $serviceDir "aion-cr.xml") -Encoding UTF8

    # Linux systemd service
    $linuxService = @"
[Unit]
Description=AION-CR Regulatory Compliance System
Documentation=https://docs.aion-cr.ai
After=network.target postgresql.service redis.service
Wants=postgresql.service redis.service

[Service]
Type=exec
User=aion
Group=aion
ExecStart=/usr/local/bin/aion-server --config /etc/aion-cr/production.toml
ExecReload=/bin/kill -HUP $MAINPID
KillMode=mixed
TimeoutStopSec=30
Restart=always
RestartSec=5
Environment=AION_ENV=production
Environment=RUST_LOG=info
WorkingDirectory=/var/lib/aion-cr
StateDirectory=aion-cr
ConfigurationDirectory=aion-cr
LogsDirectory=aion-cr
RuntimeDirectory=aion-cr

# Security settings
NoNewPrivileges=true
PrivateTmp=true
PrivateDevices=true
ProtectHome=true
ProtectSystem=strict
ReadWritePaths=/var/lib/aion-cr /var/log/aion-cr /run/aion-cr

[Install]
WantedBy=multi-user.target
"@

    $linuxService | Out-File -FilePath (Join-Path $serviceDir "aion-cr.service") -Encoding UTF8

    # macOS LaunchDaemon
    $macosService = @"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>ai.aion-cr.server</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/aion-server</string>
        <string>--config</string>
        <string>/usr/local/etc/aion-cr/production.toml</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/usr/local/var/log/aion-cr/stdout.log</string>
    <key>StandardErrorPath</key>
    <string>/usr/local/var/log/aion-cr/stderr.log</string>
    <key>EnvironmentVariables</key>
    <dict>
        <key>AION_ENV</key>
        <string>production</string>
        <key>RUST_LOG</key>
        <string>info</string>
    </dict>
    <key>WorkingDirectory</key>
    <string>/usr/local/var/lib/aion-cr</string>
</dict>
</plist>
"@

    $macosService | Out-File -FilePath (Join-Path $serviceDir "ai.aion-cr.server.plist") -Encoding UTF8

    Write-Host "✅ Created service files" -ForegroundColor Green
}

function Create-DatabaseMigrations {
    Write-Host "📊 Creating database migration scripts..." -ForegroundColor Yellow

    $migrationDir = Join-Path $BuildDir "migrations"
    New-Item -ItemType Directory -Path $migrationDir -Force | Out-Null

    # Copy migration files
    Copy-Item (Join-Path $ProjectRoot "migrations\*") $migrationDir -Recurse -Force

    # Create database setup script
    $dbSetup = @"
#!/bin/bash
# AION-CR Database Setup Script

set -e

echo "Setting up AION-CR database..."

# Check if PostgreSQL is running
if ! pg_isready -h localhost -p 5432; then
    echo "Error: PostgreSQL is not running"
    exit 1
fi

# Create database and user
sudo -u postgres psql << EOF
CREATE DATABASE aion_cr;
CREATE USER aion WITH ENCRYPTED PASSWORD 'aion_secure_password';
GRANT ALL PRIVILEGES ON DATABASE aion_cr TO aion;
ALTER USER aion CREATEDB;
EOF

# Run migrations
export DATABASE_URL="postgresql://aion:aion_secure_password@localhost:5432/aion_cr"
for migration in migrations/*.sql; do
    echo "Running migration: $migration"
    psql $DATABASE_URL -f "$migration"
done

echo "Database setup completed successfully!"
"@

    $dbSetup | Out-File -FilePath (Join-Path $migrationDir "setup.sh") -Encoding UTF8

    Write-Host "✅ Created database migration scripts" -ForegroundColor Green
}

function Create-WindowsInstaller {
    param([string]$TargetName, [hashtable]$TargetConfig)

    Write-Host "🪟 Creating Windows installer for $TargetName..." -ForegroundColor Yellow

    $nsisScript = @"
; AION-CR Windows Installer
!define PRODUCT_NAME "AION-CR"
!define PRODUCT_VERSION "$Version"
!define PRODUCT_PUBLISHER "AION-CR Team"
!define PRODUCT_WEB_SITE "https://aion-cr.ai"
!define PRODUCT_DIR_REGKEY "Software\Microsoft\Windows\CurrentVersion\App Paths\aion-server.exe"
!define PRODUCT_UNINST_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${PRODUCT_NAME}"

!include "MUI2.nsh"
!include "LogicLib.nsh"
!include "WinVer.nsh"
!include "x64.nsh"

; Modern UI Configuration
!define MUI_ABORTWARNING
!define MUI_ICON "assets\icon.ico"
!define MUI_UNICON "assets\uninstall.ico"
!define MUI_WELCOMEFINISHPAGE_BITMAP "assets\welcome.bmp"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "assets\header.bmp"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_STARTMENU startmenu `${PRODUCT_NAME}
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_RUN "`$INSTDIR\aion-server.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Start AION-CR Service"
!define MUI_FINISHPAGE_SHOWREADME "`$INSTDIR\README.md"
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

; Languages
!insertmacro MUI_LANGUAGE "English"

; General settings
Name "`${PRODUCT_NAME} `${PRODUCT_VERSION}"
OutFile "`${InstallerDir}\aion-cr-`${PRODUCT_VERSION}-$TargetName-setup.exe"
InstallDir "`$PROGRAMFILES64\AION-CR"
InstallDirRegKey HKLM "`${PRODUCT_DIR_REGKEY}" ""
ShowInstDetails show
ShowUnInstDetails show

; Version info
VIProductVersion "$Version.0"
VIAddVersionKey ProductName "`${PRODUCT_NAME}"
VIAddVersionKey ProductVersion "`${PRODUCT_VERSION}"
VIAddVersionKey CompanyName "`${PRODUCT_PUBLISHER}"
VIAddVersionKey FileDescription "AION-CR Regulatory Compliance System Installer"
VIAddVersionKey FileVersion "`${PRODUCT_VERSION}"
VIAddVersionKey LegalCopyright "© 2024 AION-CR Team"

; Sections
Section "Core Application" SEC01
    SectionIn RO
    SetOutPath "`$INSTDIR"

    ; Copy binaries
    File "build\$TargetName\binaries\aion-server.exe"
    File "build\$TargetName\binaries\aion-cli.exe"

    ; Copy web UI
    SetOutPath "`$INSTDIR\web-ui"
    File /r "build\web-ui\*"

    ; Copy configuration
    SetOutPath "`$INSTDIR\config"
    File "build\config\*"

    ; Copy service files
    SetOutPath "`$INSTDIR\service"
    File "build\services\aion-cr.xml"

    ; Create data directories
    CreateDirectory "`$INSTDIR\data"
    CreateDirectory "`$INSTDIR\logs"
    CreateDirectory "`$INSTDIR\models"

    ; Registry entries
    WriteRegStr HKLM "`${PRODUCT_DIR_REGKEY}" "" "`$INSTDIR\aion-server.exe"
    WriteRegStr HKLM "`${PRODUCT_UNINST_KEY}" "DisplayName" "`${PRODUCT_NAME}"
    WriteRegStr HKLM "`${PRODUCT_UNINST_KEY}" "UninstallString" "`$INSTDIR\uninst.exe"
    WriteRegStr HKLM "`${PRODUCT_UNINST_KEY}" "DisplayVersion" "`${PRODUCT_VERSION}"
    WriteRegStr HKLM "`${PRODUCT_UNINST_KEY}" "URLInfoAbout" "`${PRODUCT_WEB_SITE}"
    WriteRegStr HKLM "`${PRODUCT_UNINST_KEY}" "Publisher" "`${PRODUCT_PUBLISHER}"

    ; Create uninstaller
    WriteUninstaller "`$INSTDIR\uninst.exe"
SectionEnd

Section "Windows Service" SEC02
    ; Install Windows service
    ExecWait '"`$INSTDIR\aion-server.exe" service install --config "`$INSTDIR\config\production.toml"'
SectionEnd

Section "Desktop Shortcut" SEC03
    CreateShortCut "`$DESKTOP\AION-CR.lnk" "`$INSTDIR\aion-cli.exe"
SectionEnd

Section "Start Menu Shortcuts" SEC04
    !insertmacro MUI_STARTMENU_WRITE_BEGIN startmenu
    CreateDirectory "`$SMPROGRAMS\`$startmenu"
    CreateShortCut "`$SMPROGRAMS\`$startmenu\AION-CR CLI.lnk" "`$INSTDIR\aion-cli.exe"
    CreateShortCut "`$SMPROGRAMS\`$startmenu\AION-CR Web Interface.lnk" "http://localhost:8080"
    CreateShortCut "`$SMPROGRAMS\`$startmenu\Uninstall.lnk" "`$INSTDIR\uninst.exe"
    !insertmacro MUI_STARTMENU_WRITE_END
SectionEnd

Section "Add to PATH" SEC05
    EnVar::SetHKLM
    EnVar::AddValue "PATH" "`$INSTDIR"
SectionEnd

; Uninstaller
Section Uninstall
    ; Stop and remove service
    ExecWait '"`$INSTDIR\aion-server.exe" service stop'
    ExecWait '"`$INSTDIR\aion-server.exe" service uninstall'

    ; Remove files
    Delete "`$INSTDIR\aion-server.exe"
    Delete "`$INSTDIR\aion-cli.exe"
    Delete "`$INSTDIR\uninst.exe"
    RMDir /r "`$INSTDIR\web-ui"
    RMDir /r "`$INSTDIR\config"
    RMDir /r "`$INSTDIR\service"
    RMDir /r "`$INSTDIR\models"

    ; Remove shortcuts
    Delete "`$DESKTOP\AION-CR.lnk"
    !insertmacro MUI_STARTMENU_GETFOLDER startmenu `$startmenu
    Delete "`$SMPROGRAMS\`$startmenu\*"
    RMDir "`$SMPROGRAMS\`$startmenu"

    ; Remove from PATH
    EnVar::SetHKLM
    EnVar::DeleteValue "PATH" "`$INSTDIR"

    ; Remove registry entries
    DeleteRegKey HKLM "`${PRODUCT_UNINST_KEY}"
    DeleteRegKey HKLM "`${PRODUCT_DIR_REGKEY}"

    ; Remove installation directory
    RMDir "`$INSTDIR"
SectionEnd
"@

    $nsisScript | Out-File -FilePath (Join-Path $InstallerDir "installer-$TargetName.nsi") -Encoding UTF8

    # Build installer with NSIS
    if (Get-Command "makensis" -ErrorAction SilentlyContinue) {
        Push-Location $InstallerDir
        makensis "installer-$TargetName.nsi"
        Pop-Location
        Write-Host "✅ Created Windows installer for $TargetName" -ForegroundColor Green
    } else {
        Write-Warning "NSIS not found. Please install NSIS to build Windows installers."
    }
}

function Create-MacOSInstaller {
    param([string]$TargetName, [hashtable]$TargetConfig)

    Write-Host "🍎 Creating macOS installer for $TargetName..." -ForegroundColor Yellow

    $pkgRoot = Join-Path $InstallerDir "macos-$TargetName"
    $appRoot = Join-Path $pkgRoot "Applications\AION-CR.app"

    # Create app bundle structure
    New-Item -ItemType Directory -Path "$appRoot\Contents\MacOS" -Force | Out-Null
    New-Item -ItemType Directory -Path "$appRoot\Contents\Resources" -Force | Out-Null

    # Copy binaries
    Copy-Item "build\$TargetName\binaries\aion-server" "$appRoot\Contents\MacOS\" -Force
    Copy-Item "build\$TargetName\binaries\aion-cli" "$appRoot\Contents\MacOS\" -Force

    # Copy web UI and config
    Copy-Item "build\web-ui" "$appRoot\Contents\Resources\" -Recurse -Force
    Copy-Item "build\config" "$appRoot\Contents\Resources\" -Recurse -Force

    # Create Info.plist
    $infoPlist = @"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>aion-server</string>
    <key>CFBundleIdentifier</key>
    <string>ai.aion-cr.server</string>
    <key>CFBundleName</key>
    <string>AION-CR</string>
    <key>CFBundleVersion</key>
    <string>$Version</string>
    <key>CFBundleShortVersionString</key>
    <string>$Version</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>AION</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSUIElement</key>
    <true/>
</dict>
</plist>
"@

    $infoPlist | Out-File -FilePath "$appRoot\Contents\Info.plist" -Encoding UTF8

    # Create PKG installer
    if (Get-Command "pkgbuild" -ErrorAction SilentlyContinue) {
        $pkgPath = Join-Path $InstallerDir "aion-cr-$Version-$TargetName.pkg"
        pkgbuild --root $pkgRoot --identifier "ai.aion-cr.installer" --version $Version $pkgPath
        Write-Host "✅ Created macOS installer for $TargetName" -ForegroundColor Green
    } else {
        Write-Warning "pkgbuild not found. Please run on macOS to build PKG installers."
    }
}

function Create-LinuxInstaller {
    param([string]$TargetName, [hashtable]$TargetConfig)

    Write-Host "🐧 Creating Linux installer for $TargetName..." -ForegroundColor Yellow

    $debRoot = Join-Path $InstallerDir "linux-$TargetName"
    $debianDir = Join-Path $debRoot "DEBIAN"

    # Create package structure
    New-Item -ItemType Directory -Path $debianDir -Force | Out-Null
    New-Item -ItemType Directory -Path "$debRoot\usr\local\bin" -Force | Out-Null
    New-Item -ItemType Directory -Path "$debRoot\etc\aion-cr" -Force | Out-Null
    New-Item -ItemType Directory -Path "$debRoot\var\lib\aion-cr" -Force | Out-Null
    New-Item -ItemType Directory -Path "$debRoot\var\log\aion-cr" -Force | Out-Null
    New-Item -ItemType Directory -Path "$debRoot\etc\systemd\system" -Force | Out-Null

    # Copy binaries
    Copy-Item "build\$TargetName\binaries\aion-server" "$debRoot\usr\local\bin\" -Force
    Copy-Item "build\$TargetName\binaries\aion-cli" "$debRoot\usr\local\bin\" -Force

    # Copy configuration
    Copy-Item "build\config\*" "$debRoot\etc\aion-cr\" -Force

    # Copy systemd service
    Copy-Item "build\services\aion-cr.service" "$debRoot\etc\systemd\system\" -Force

    # Create control file
    $control = @"
Package: aion-cr
Version: $Version
Section: utils
Priority: optional
Architecture: amd64
Depends: postgresql-client, redis-tools, curl
Maintainer: AION-CR Team <support@aion-cr.ai>
Description: Advanced AI-powered regulatory compliance management system
 AION-CR is a comprehensive regulatory compliance management system that uses
 advanced AI and machine learning to monitor, assess, and ensure compliance
 across multiple regulatory frameworks.
 .
 Features:
  - Real-time regulatory monitoring
  - AI-powered conflict detection and resolution
  - Autonomous compliance agents
  - Advanced ML/NLP engines
  - Comprehensive reporting and analytics
Homepage: https://aion-cr.ai
"@

    $control | Out-File -FilePath "$debianDir\control" -Encoding UTF8

    # Create postinst script
    $postinst = @"
#!/bin/bash
set -e

# Create aion user
if ! id -u aion >/dev/null 2>&1; then
    useradd --system --home /var/lib/aion-cr --shell /bin/false aion
fi

# Set permissions
chown -R aion:aion /var/lib/aion-cr
chown -R aion:aion /var/log/aion-cr
chmod +x /usr/local/bin/aion-server
chmod +x /usr/local/bin/aion-cli

# Enable and start service
systemctl daemon-reload
systemctl enable aion-cr
systemctl start aion-cr

echo "AION-CR installed successfully!"
echo "Access the web interface at: http://localhost:8080"
echo "Use 'aion-cli --help' for command-line usage"
"@

    $postinst | Out-File -FilePath "$debianDir\postinst" -Encoding UTF8

    # Create prerm script
    $prerm = @"
#!/bin/bash
set -e

# Stop and disable service
systemctl stop aion-cr || true
systemctl disable aion-cr || true
"@

    $prerm | Out-File -FilePath "$debianDir\prerm" -Encoding UTF8

    # Build DEB package
    if (Get-Command "dpkg-deb" -ErrorAction SilentlyContinue) {
        $debPath = Join-Path $InstallerDir "aion-cr_$Version`_amd64.deb"
        dpkg-deb --build $debRoot $debPath
        Write-Host "✅ Created Linux installer for $TargetName" -ForegroundColor Green
    } else {
        Write-Warning "dpkg-deb not found. Please run on Linux to build DEB packages."
    }
}

function Sign-Installers {
    if (-not $Sign) { return }

    Write-Host "🔐 Signing installers..." -ForegroundColor Yellow

    # Sign Windows installers
    Get-ChildItem $InstallerDir -Filter "*.exe" | ForEach-Object {
        if (Get-Command "signtool" -ErrorAction SilentlyContinue) {
            signtool sign /f $env:SIGNING_CERT /p $env:SIGNING_PASSWORD /t http://timestamp.sectigo.com $_.FullName
            Write-Host "✅ Signed $($_.Name)" -ForegroundColor Green
        }
    }

    # Sign macOS installers
    Get-ChildItem $InstallerDir -Filter "*.pkg" | ForEach-Object {
        if (Get-Command "productsign" -ErrorAction SilentlyContinue) {
            productsign --sign "Developer ID Installer" $_.FullName "$($_.BaseName)-signed.pkg"
            Write-Host "✅ Signed $($_.Name)" -ForegroundColor Green
        }
    }
}

function Create-Checksums {
    Write-Host "🔍 Creating checksums..." -ForegroundColor Yellow

    $checksumFile = Join-Path $InstallerDir "checksums.txt"
    "" | Out-File -FilePath $checksumFile -Encoding UTF8

    Get-ChildItem $InstallerDir -File | ForEach-Object {
        if ($_.Extension -in @(".exe", ".pkg", ".deb", ".zip", ".tar.gz")) {
            $hash = Get-FileHash $_.FullName -Algorithm SHA256
            "$($hash.Hash)  $($_.Name)" | Add-Content -Path $checksumFile -Encoding UTF8
        }
    }

    Write-Host "✅ Created checksums file" -ForegroundColor Green
}

function Upload-Artifacts {
    if (-not $Upload) { return }

    Write-Host "☁️ Uploading artifacts..." -ForegroundColor Yellow

    # Upload to GitHub Releases
    if ($env:GITHUB_TOKEN) {
        $releaseTag = "v$Version"

        # Create release if it doesn't exist
        try {
            gh release view $releaseTag 2>$null
        } catch {
            gh release create $releaseTag --title "AION-CR $Version" --notes "Release $Version"
        }

        # Upload artifacts
        Get-ChildItem $InstallerDir -File | ForEach-Object {
            if ($_.Extension -in @(".exe", ".pkg", ".deb", ".zip", ".tar.gz", ".txt")) {
                gh release upload $releaseTag $_.FullName --clobber
                Write-Host "✅ Uploaded $($_.Name)" -ForegroundColor Green
            }
        }
    }

    # Upload to cloud storage (if configured)
    if ($env:AWS_ACCESS_KEY_ID -and $env:AWS_SECRET_ACCESS_KEY) {
        aws s3 sync $InstallerDir "s3://aion-cr-releases/$Version/" --exclude "*" --include "*.exe" --include "*.pkg" --include "*.deb" --include "checksums.txt"
        Write-Host "✅ Uploaded to S3" -ForegroundColor Green
    }
}

# Main execution
try {
    # Determine targets to build
    $targetsToProcess = if ($Target -eq "all") {
        $BuildTargets
    } else {
        @{ $Target = $BuildTargets[$Target] }
    }

    if (-not $targetsToProcess) {
        Write-Error "Invalid target: $Target. Available targets: $($BuildTargets.Keys -join ', ')"
        exit 1
    }

    Write-Host "Building targets: $($targetsToProcess.Keys -join ', ')" -ForegroundColor Cyan

    # Build components
    Build-RustBinaries -Targets $targetsToProcess
    Build-WebUI
    Create-ConfigFiles
    Create-ServiceFiles
    Create-DatabaseMigrations

    # Create installers for each target
    foreach ($targetName in $targetsToProcess.Keys) {
        $targetConfig = $targetsToProcess[$targetName]

        switch ($targetConfig.Platform) {
            "windows" { Create-WindowsInstaller -TargetName $targetName -TargetConfig $targetConfig }
            "macos" { Create-MacOSInstaller -TargetName $targetName -TargetConfig $targetConfig }
            "linux" { Create-LinuxInstaller -TargetName $targetName -TargetConfig $targetConfig }
        }
    }

    # Post-processing
    Sign-Installers
    Create-Checksums
    Upload-Artifacts

    Write-Host ""
    Write-Host "🎉 Build completed successfully!" -ForegroundColor Green
    Write-Host "Installers created in: $InstallerDir" -ForegroundColor Cyan
    Write-Host "Version: $Version" -ForegroundColor Cyan
    Write-Host "Targets: $($targetsToProcess.Keys -join ', ')" -ForegroundColor Cyan
}
catch {
    Write-Host "❌ Build failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}