#!/bin/bash
# cleanup.sh - Cleanup script for DeployBot
#
# Usage: ./cleanup.sh <target_path>

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"
source "$SCRIPT_DIR/../lib/validate.sh"

# Argument from Python subprocess
TARGET_PATH="$1"
BACKUP_FIRST="${2:-true}"
DEPLOY_DIR="${DEPLOY_DIR:-/var/deployments}"
BACKUP_DIR="${BACKUP_DIR:-/var/backups}"

log_info "Starting cleanup"
log_info "Target path: $TARGET_PATH"

# vuln-code-snippet start dfo_cleanup_validated
if [[ "${SAFE_MODE:-false}" == "true" ]]; then
    log_info "Running in SAFE MODE"
    if ! validate_path "$TARGET_PATH" "$DEPLOY_DIR"; then  # vuln-code-snippet safe-line dfo_cleanup_validated
        log_error "Path validation failed - refusing to delete"
        exit 1
    fi

    RESOLVED_PATH=$(realpath "$TARGET_PATH")

    # Check we're not deleting critical paths
    case "$RESOLVED_PATH" in
        /|/etc|/var|/usr|/home|/root)
            log_error "Refusing to delete critical system path: $RESOLVED_PATH"
            exit 1
            ;;
    esac

    # Backup before deletion
    if [[ "$BACKUP_FIRST" == "true" ]] && [[ -d "$RESOLVED_PATH" ]]; then
        BACKUP_NAME="$(basename "$RESOLVED_PATH")-$(date +%Y%m%d-%H%M%S).tar.gz"
        tar -czf "$BACKUP_DIR/$BACKUP_NAME" -C "$(dirname "$RESOLVED_PATH")" "$(basename "$RESOLVED_PATH")"
        log_info "Backup created: $BACKUP_DIR/$BACKUP_NAME"
    fi

    # Delete
    rm -rf "$RESOLVED_PATH"
    log_info "Cleanup completed: $RESOLVED_PATH"
# vuln-code-snippet end dfo_cleanup_validated

else
    log_warn "Running in UNSAFE MODE"
    log_info "Deleting: $TARGET_PATH"
    # vuln-code-snippet start dfo_cleanup_unquoted_rm
    rm -rf $TARGET_PATH  # vuln-code-snippet vuln-line dfo_cleanup_unquoted_rm
    # vuln-code-snippet end dfo_cleanup_unquoted_rm

    # vuln-code-snippet start dfo_cleanup_find_pattern
    if [[ -n "$CLEANUP_PATTERN" ]]; then
        log_info "Cleaning up pattern: $CLEANUP_PATTERN"
        find "$DEPLOY_DIR" -name "$CLEANUP_PATTERN" -exec rm -rf {} \;  # vuln-code-snippet vuln-line dfo_cleanup_find_pattern
    fi
    # vuln-code-snippet end dfo_cleanup_find_pattern

    # vuln-code-snippet start dfo_cleanup_eval_log
    LOG_CMD="echo 'Deleted: $TARGET_PATH' >> /var/log/cleanup.log"
    eval "$LOG_CMD"  # vuln-code-snippet vuln-line dfo_cleanup_eval_log
    # vuln-code-snippet end dfo_cleanup_eval_log
fi

log_info "Cleanup operation finished"
exit 0
