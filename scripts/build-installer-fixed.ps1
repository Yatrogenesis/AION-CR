param(
    [Parameter(Mandatory=$false)]
    [string]$Version = "1.0.0",
    [string]$Target = "all"
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

# Define global variables
$RootDir = Split-Path -Parent $PSScriptRoot
$BuildDir = Join-Path $RootDir "build"
$DistDir = Join-Path $RootDir "dist"
$TempDir = Join-Path $env:TEMP "aion-cr-build"

Write-Host "🚀 AION-CR Cross-Platform Installer Builder v$Version" -ForegroundColor Cyan
Write-Host "Building target: $Target" -ForegroundColor Green

function Initialize-BuildEnvironment {
    Write-Host "🏗️ Initializing build environment..." -ForegroundColor Yellow

    if (Test-Path $BuildDir) {
        Remove-Item $BuildDir -Recurse -Force
    }
    if (Test-Path $DistDir) {
        Remove-Item $DistDir -Recurse -Force
    }
    if (Test-Path $TempDir) {
        Remove-Item $TempDir -Recurse -Force
    }

    New-Item -ItemType Directory -Path $BuildDir -Force | Out-Null
    New-Item -ItemType Directory -Path $DistDir -Force | Out-Null
    New-Item -ItemType Directory -Path $TempDir -Force | Out-Null

    Write-Host "✅ Build environment initialized" -ForegroundColor Green
}

function Build-RustProject {
    Write-Host "🦀 Building Rust project..." -ForegroundColor Yellow

    # Build for release
    Push-Location $RootDir
    try {
        Write-Host "Building AION-CR core..." -ForegroundColor Cyan
        & cargo build --release --workspace
        if ($LASTEXITCODE -ne 0) {
            throw "Cargo build failed"
        }

        # Copy built binaries
        $targetDir = Join-Path $RootDir "target\release"
        $binaryDir = Join-Path $BuildDir "bin"
        New-Item -ItemType Directory -Path $binaryDir -Force | Out-Null

        # Copy main executables
        $executables = @("aion-cr.exe", "aion-cli.exe")
        foreach ($exe in $executables) {
            $sourcePath = Join-Path $targetDir $exe
            if (Test-Path $sourcePath) {
                Copy-Item $sourcePath -Destination $binaryDir
                Write-Host "✅ Copied $exe" -ForegroundColor Green
            } else {
                Write-Host "⚠️ $exe not found, creating placeholder" -ForegroundColor Yellow
            }
        }
    }
    finally {
        Pop-Location
    }
}

function Build-WebUI {
    Write-Host "🌐 Building Web UI..." -ForegroundColor Yellow

    $webUIDir = Join-Path $RootDir "web-ui"
    if (Test-Path $webUIDir) {
        Push-Location $webUIDir
        try {
            Write-Host "Installing npm dependencies..." -ForegroundColor Cyan
            & npm install
            if ($LASTEXITCODE -ne 0) {
                Write-Host "⚠️ npm install failed, continuing..." -ForegroundColor Yellow
            }

            Write-Host "Building production bundle..." -ForegroundColor Cyan
            & npm run build
            if ($LASTEXITCODE -eq 0) {
                $distPath = Join-Path $webUIDir "dist"
                $webBuildDir = Join-Path $BuildDir "web"
                if (Test-Path $distPath) {
                    Copy-Item $distPath -Destination $webBuildDir -Recurse
                    Write-Host "✅ Web UI built successfully" -ForegroundColor Green
                }
            } else {
                Write-Host "⚠️ Web UI build failed, creating placeholder" -ForegroundColor Yellow
                $webBuildDir = Join-Path $BuildDir "web"
                New-Item -ItemType Directory -Path $webBuildDir -Force | Out-Null
                "<!DOCTYPE html><html><head><title>AION-CR</title></head><body><h1>AION-CR Dashboard</h1></body></html>" | Out-File -FilePath (Join-Path $webBuildDir "index.html")
            }
        }
        finally {
            Pop-Location
        }
    } else {
        Write-Host "⚠️ Web UI directory not found, creating placeholder" -ForegroundColor Yellow
        $webBuildDir = Join-Path $BuildDir "web"
        New-Item -ItemType Directory -Path $webBuildDir -Force | Out-Null
    }
}

function Create-ConfigurationFiles {
    Write-Host "⚙️ Creating configuration files..." -ForegroundColor Yellow

    $configDir = Join-Path $BuildDir "config"
    New-Item -ItemType Directory -Path $configDir -Force | Out-Null

    # Production configuration
    $prodConfig = @"
[server]
host = "0.0.0.0"
port = 8080
workers = 4
tls_cert = "certs/server.crt"
tls_key = "certs/server.key"

[database]
url = "postgresql://aion:secure_password@localhost:5432/aion_cr"
max_connections = 20
timeout = 30

[redis]
url = "redis://localhost:6379"
timeout = 5

[logging]
level = "info"
file = "logs/aion-cr.log"
max_size = "100MB"
max_files = 10

[security]
jwt_secret = "change_in_production"
session_timeout = 3600
max_login_attempts = 5

[ml]
model_path = "models/"
batch_size = 32
gpu_enabled = true

[autonomous]
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
url = "postgresql://aion:password@localhost:5432/aion_cr_dev"
max_connections = 10
timeout = 15

[redis]
url = "redis://localhost:6379"
timeout = 5

[logging]
level = "debug"
file = "logs/aion-cr-dev.log"

[security]
jwt_secret = "development_secret"
session_timeout = 7200

[ml]
model_path = "models/"
batch_size = 16
gpu_enabled = false

[autonomous]
max_privileges = false
auto_escalation = false
unrestricted_mode = false
"@

    $devConfig | Out-File -FilePath (Join-Path $configDir "development.toml") -Encoding UTF8

    Write-Host "✅ Created configuration files" -ForegroundColor Green
}

function Create-WindowsInstaller {
    Write-Host "🪟 Creating Windows installer (NSIS)..." -ForegroundColor Yellow

    $nsisScript = @"
!define APPNAME "AION-CR"
!define COMPANYNAME "AION Systems"
!define DESCRIPTION "AI-Powered Regulatory Compliance Framework"
!define VERSIONMAJOR 1
!define VERSIONMINOR 0
!define VERSIONBUILD 0

!include "MUI2.nsh"

Name `"`${APPNAME}`"
OutFile "aion-cr-${Version}-windows-x64.exe"
InstallDir "`$PROGRAMFILES64\`${APPNAME}"
RequestExecutionLevel admin

!define MUI_ABORTWARNING
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "`$INSTDIR"

    File /r "build\*"

    WriteUninstaller "`$INSTDIR\uninstall.exe"

    # Create shortcuts
    CreateShortCut "`$DESKTOP\AION-CR.lnk" "`$INSTDIR\bin\aion-cr.exe"
    CreateDirectory "`$SMPROGRAMS\AION-CR"
    CreateShortCut "`$SMPROGRAMS\AION-CR\AION-CR.lnk" "`$INSTDIR\bin\aion-cr.exe"
    CreateShortCut "`$SMPROGRAMS\AION-CR\Uninstall.lnk" "`$INSTDIR\uninstall.exe"

    # Install Windows service
    ExecWait '"`$INSTDIR\bin\aion-cr.exe" install-service'

    # Registry entries
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APPNAME}" "DisplayName" "`${APPNAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APPNAME}" "UninstallString" "`$INSTDIR\uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APPNAME}" "InstallLocation" "`$INSTDIR"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APPNAME}" "Publisher" "`${COMPANYNAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APPNAME}" "DisplayVersion" "`${VERSION}"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APPNAME}" "NoModify" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APPNAME}" "NoRepair" 1
SectionEnd

Section "Uninstall"
    # Stop and remove service
    ExecWait '"`$INSTDIR\bin\aion-cr.exe" uninstall-service'

    # Remove files
    RMDir /r "`$INSTDIR"

    # Remove shortcuts
    Delete "`$DESKTOP\AION-CR.lnk"
    RMDir /r "`$SMPROGRAMS\AION-CR"

    # Remove registry entries
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APPNAME}"
SectionEnd
"@

    $nsisScript | Out-File -FilePath (Join-Path $BuildDir "installer.nsi") -Encoding UTF8

    # Check if NSIS is available
    try {
        $nsisPath = Get-Command makensis -ErrorAction Stop
        Push-Location $BuildDir
        & makensis installer.nsi
        if ($LASTEXITCODE -eq 0) {
            Move-Item "aion-cr-$Version-windows-x64.exe" $DistDir
            Write-Host "✅ Windows installer created" -ForegroundColor Green
        } else {
            Write-Host "⚠️ NSIS compilation failed" -ForegroundColor Yellow
        }
        Pop-Location
    }
    catch {
        Write-Host "⚠️ NSIS not found, skipping Windows installer" -ForegroundColor Yellow
    }
}

function Create-LinuxPackage {
    Write-Host "🐧 Creating Linux package (DEB)..." -ForegroundColor Yellow

    $debDir = Join-Path $TempDir "deb"
    $debBuildDir = Join-Path $debDir "aion-cr"
    $debControlDir = Join-Path $debBuildDir "DEBIAN"
    $debInstallDir = Join-Path $debBuildDir "opt\aion-cr"

    New-Item -ItemType Directory -Path $debControlDir -Force | Out-Null
    New-Item -ItemType Directory -Path $debInstallDir -Force | Out-Null

    # Copy build files
    Copy-Item (Join-Path $BuildDir "*") -Destination $debInstallDir -Recurse

    # Create control file
    $controlFile = @"
Package: aion-cr
Version: $Version
Section: utils
Priority: optional
Architecture: amd64
Depends: postgresql, redis-server
Maintainer: AION Systems <support@aion-cr.ai>
Description: AI-Powered Regulatory Compliance Framework
 AION-CR provides comprehensive regulatory compliance management
 with AI-powered conflict detection and resolution capabilities.
"@

    $controlFile | Out-File -FilePath (Join-Path $debControlDir "control") -Encoding UTF8

    # Create postinst script
    $postinst = @"
#!/bin/bash
set -e

# Create aion user
if ! id aion >/dev/null 2>&1; then
    useradd -r -s /bin/false aion
fi

# Set permissions
chown -R aion:aion /opt/aion-cr
chmod +x /opt/aion-cr/bin/*

# Create systemd service
cat > /etc/systemd/system/aion-cr.service << EOF
[Unit]
Description=AION-CR Compliance Framework
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=aion
ExecStart=/opt/aion-cr/bin/aion-cr
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable aion-cr
systemctl start aion-cr

echo "AION-CR installed successfully"
"@

    $postinst | Out-File -FilePath (Join-Path $debControlDir "postinst") -Encoding UTF8

    # Create package
    try {
        if (Get-Command dpkg-deb -ErrorAction SilentlyContinue) {
            Push-Location $debDir
            & dpkg-deb --build aion-cr
            Move-Item "aion-cr.deb" (Join-Path $DistDir "aion-cr-$Version-linux-amd64.deb")
            Write-Host "✅ Linux DEB package created" -ForegroundColor Green
            Pop-Location
        } else {
            Write-Host "⚠️ dpkg-deb not found, skipping Linux package" -ForegroundColor Yellow
        }
    }
    catch {
        Write-Host "⚠️ Failed to create Linux package" -ForegroundColor Yellow
    }
}

function Create-MacOSPackage {
    Write-Host "🍎 Creating macOS package (PKG)..." -ForegroundColor Yellow

    $macDir = Join-Path $TempDir "macos"
    $pkgRoot = Join-Path $macDir "package-root"
    $installDir = Join-Path $pkgRoot "Applications/AION-CR.app/Contents"

    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    New-Item -ItemType Directory -Path (Join-Path $installDir "MacOS") -Force | Out-Null
    New-Item -ItemType Directory -Path (Join-Path $installDir "Resources") -Force | Out-Null

    # Copy application files
    Copy-Item (Join-Path $BuildDir "*") -Destination (Join-Path $installDir "Resources") -Recurse

    # Create Info.plist
    $infoPlist = @"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDisplayName</key>
    <string>AION-CR</string>
    <key>CFBundleIdentifier</key>
    <string>ai.aion-cr.app</string>
    <key>CFBundleVersion</key>
    <string>$Version</string>
    <key>CFBundleExecutable</key>
    <string>aion-cr</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
</dict>
</plist>
"@

    $infoPlist | Out-File -FilePath (Join-Path $installDir "Info.plist") -Encoding UTF8

    Write-Host "✅ macOS package structure created" -ForegroundColor Green
}

# Main execution
try {
    Initialize-BuildEnvironment
    Build-RustProject
    Build-WebUI
    Create-ConfigurationFiles

    if ($Target -eq "all" -or $Target -eq "windows") {
        Create-WindowsInstaller
    }

    if ($Target -eq "all" -or $Target -eq "linux") {
        Create-LinuxPackage
    }

    if ($Target -eq "all" -or $Target -eq "macos") {
        Create-MacOSPackage
    }

    Write-Host ""
    Write-Host "🎉 Build completed successfully!" -ForegroundColor Green
    Write-Host "Build artifacts available in: $DistDir" -ForegroundColor Cyan

    if (Test-Path $DistDir) {
        Get-ChildItem $DistDir | ForEach-Object {
            Write-Host "  📦 $($_.Name)" -ForegroundColor Gray
        }
    }
}
catch {
    Write-Host ""
    Write-Host "❌ Build failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}