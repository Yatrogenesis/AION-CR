#!/bin/bash

# AION-CR ↔ ECTUS-R Unified System Startup Script
# Maximum Autonomy and Privilege Escalation

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
INTEGRATION_CONFIG="$PROJECT_ROOT/config/integration.toml"
LOG_DIR="/var/log/aion-integration"
PID_FILE="/var/run/aion-integration.pid"

# Logging function
log() {
    echo -e "${CYAN}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

# Check if running as root for maximum privileges
check_privileges() {
    if [[ $EUID -eq 0 ]]; then
        success "Running with maximum privileges (root)"
        export AION_MAX_PRIVILEGES=true
    else
        warning "Not running as root. Some features may be limited."
        warning "For maximum autonomy, run with: sudo $0"
        export AION_MAX_PRIVILEGES=false
    fi
}

# Setup environment
setup_environment() {
    log "Setting up environment for unified system"

    # Create log directory
    if [[ ! -d "$LOG_DIR" ]]; then
        sudo mkdir -p "$LOG_DIR"
        sudo chmod 755 "$LOG_DIR"
        if [[ $EUID -eq 0 ]]; then
            chown "$(whoami):$(whoami)" "$LOG_DIR"
        fi
    fi

    # Set environment variables
    export RUST_LOG=debug
    export AION_INTEGRATION_CONFIG="$INTEGRATION_CONFIG"
    export AION_LOG_DIR="$LOG_DIR"
    export AION_AUTONOMY_LEVEL=255
    export AION_PRIVILEGE_ESCALATION=enabled
    export AION_UNRESTRICTED_MODE=true
    export CLAUDE_CODE_MAX_OUTPUT_TOKENS=128000

    # PostgreSQL configuration
    export PGUSER="${PGUSER:-aion}"
    export PGPASSWORD="${PGPASSWORD:-secure}"
    export PGHOST="${PGHOST:-localhost}"
    export PGPORT="${PGPORT:-5432}"
    export PGDATABASE="${PGDATABASE:-aion_cr}"

    # Redis configuration
    export REDIS_URL="${REDIS_URL:-redis://localhost:6379/0}"

    # Regulatory API keys (set these in your environment)
    if [[ -z "${FERC_API_KEY:-}" ]]; then
        warning "FERC_API_KEY not set. FERC integration will be limited."
    fi

    if [[ -z "${NERC_API_KEY:-}" ]]; then
        warning "NERC_API_KEY not set. NERC integration will be limited."
    fi

    success "Environment configured"
}

# Check dependencies
check_dependencies() {
    log "Checking system dependencies"

    local missing_deps=()

    # Check for required commands
    local required_commands=("cargo" "rustc" "psql" "redis-cli")
    for cmd in "${required_commands[@]}"; do
        if ! command -v "$cmd" &> /dev/null; then
            missing_deps+=("$cmd")
        fi
    done

    # Check for PostgreSQL
    if ! systemctl is-active --quiet postgresql 2>/dev/null && ! service postgresql status &>/dev/null; then
        warning "PostgreSQL is not running. Attempting to start..."
        if [[ $EUID -eq 0 ]]; then
            systemctl start postgresql || service postgresql start
        else
            warning "Need root privileges to start PostgreSQL"
        fi
    fi

    # Check for Redis
    if ! systemctl is-active --quiet redis 2>/dev/null && ! service redis-server status &>/dev/null; then
        warning "Redis is not running. Attempting to start..."
        if [[ $EUID -eq 0 ]]; then
            systemctl start redis || service redis-server start
        else
            warning "Need root privileges to start Redis"
        fi
    fi

    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        error "Missing dependencies: ${missing_deps[*]}"
        error "Please install missing dependencies and try again"
        exit 1
    fi

    success "All dependencies satisfied"
}

# Build the system
build_system() {
    log "Building AION-CR with ECTUS-R integration"

    cd "$PROJECT_ROOT"

    # Clean previous builds
    cargo clean

    # Build in release mode for maximum performance
    info "Building in release mode (this may take a while)..."
    cargo build --release --workspace

    if [[ $? -eq 0 ]]; then
        success "Build completed successfully"
    else
        error "Build failed"
        exit 1
    fi
}

# Start the unified system
start_unified_system() {
    log "Starting unified AION-CR ↔ ECTUS-R system with maximum autonomy"

    cd "$PROJECT_ROOT"

    # Check if already running
    if [[ -f "$PID_FILE" ]] && kill -0 "$(cat "$PID_FILE")" 2>/dev/null; then
        warning "System is already running (PID: $(cat "$PID_FILE"))"
        info "Use stop_unified_system.sh to stop the current instance"
        exit 1
    fi

    # Start the system with maximum autonomy
    info "Launching unified system..."

    # Use nohup to keep running after script exits
    nohup cargo run --release --bin aion-cr -- --unified \
        > "$LOG_DIR/unified-system.log" 2>&1 &

    local pid=$!
    echo "$pid" > "$PID_FILE"

    # Give it a moment to start
    sleep 3

    # Check if it's still running
    if kill -0 "$pid" 2>/dev/null; then
        success "Unified system started successfully (PID: $pid)"
        info "Logs: $LOG_DIR/unified-system.log"
        info "Config: $INTEGRATION_CONFIG"
        info "🚀 AION-CR ↔ ECTUS-R operating as unified system with maximum autonomy"
        info "🏆 Privilege escalation enabled, unrestricted mode active"
        info ""
        info "System endpoints:"
        info "  • AION-CR API: https://aion-cr.internal:8443"
        info "  • ECTUS-R API: https://ectus-r.internal:8444"
        info "  • Unified Dashboard: https://dashboard.aion-cr.ai"
        info "  • Monitoring: https://monitoring.aion-cr.ai"
        info ""
        info "To stop the system, run: ./scripts/stop_unified_system.sh"
        info "To view logs: tail -f $LOG_DIR/unified-system.log"
        info "To check status: ./scripts/status_unified_system.sh"
    else
        error "Failed to start unified system"
        error "Check logs: $LOG_DIR/unified-system.log"
        rm -f "$PID_FILE"
        exit 1
    fi
}

# Health check
health_check() {
    log "Performing system health check"

    # Wait a bit for system to fully initialize
    sleep 10

    # Check if process is running
    if [[ -f "$PID_FILE" ]] && kill -0 "$(cat "$PID_FILE")" 2>/dev/null; then
        success "System process is running"

        # Try to connect to health endpoint (if implemented)
        if command -v curl &> /dev/null; then
            if timeout 5 curl -s -k https://localhost:5950/health >/dev/null 2>&1; then
                success "Health endpoint responding"
            else
                warning "Health endpoint not responding (may still be initializing)"
            fi
        fi
    else
        error "System process is not running"
        return 1
    fi
}

# Main execution
main() {
    echo -e "${PURPLE}"
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║                    AION-CR ↔ ECTUS-R Unified System Launcher                ║"
    echo "║                         Maximum Autonomy & Privilege Escalation             ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"

    check_privileges
    setup_environment
    check_dependencies

    # Parse command line arguments
    local skip_build=false
    for arg in "$@"; do
        case $arg in
            --skip-build)
                skip_build=true
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --skip-build    Skip the build step"
                echo "  --help, -h      Show this help message"
                echo ""
                echo "Environment Variables:"
                echo "  FERC_API_KEY    API key for FERC integration"
                echo "  NERC_API_KEY    API key for NERC integration"
                echo "  PGUSER          PostgreSQL username (default: aion)"
                echo "  PGPASSWORD      PostgreSQL password (default: secure)"
                echo "  REDIS_URL       Redis connection URL"
                echo ""
                exit 0
                ;;
            *)
                warning "Unknown argument: $arg"
                ;;
        esac
    done

    if [[ "$skip_build" == false ]]; then
        build_system
    else
        info "Skipping build step"
    fi

    start_unified_system
    health_check

    success "🎉 AION-CR ↔ ECTUS-R unified system is now operational!"
    info "The system is running in the background with maximum autonomy enabled."
}

# Handle script interruption
trap 'error "Script interrupted"; exit 130' INT TERM

# Run main function
main "$@"