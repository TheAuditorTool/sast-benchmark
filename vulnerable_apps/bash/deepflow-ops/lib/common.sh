#!/bin/bash
# Common functions and utilities for DeployBot bash scripts

# Colors for output (optional)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $(date '+%Y-%m-%d %H:%M:%S') - $1" >&2
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Get script directory
get_script_dir() {
    cd "$(dirname "${BASH_SOURCE[0]}")" && pwd
}

# Environment defaults
DEFAULT_DEPLOY_DIR="/var/deployments"
DEFAULT_BACKUP_DIR="/var/backups"
DEFAULT_LOG_DIR="/var/log/deploybot"
WEBHOOK_URL="${WEBHOOK_URL:-http://localhost:9000/hooks}"
