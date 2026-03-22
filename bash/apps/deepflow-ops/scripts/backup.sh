#!/bin/bash
# backup.sh - Backup script for DeployBot
#
# Usage: ./backup.sh <source_path> [destination]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../lib/common.sh"
source "$SCRIPT_DIR/../lib/validate.sh"

# Arguments from Python subprocess
SOURCE_PATH="$1"
DEST_PATH="${2:-$BACKUP_DIR}"
BACKUP_DIR="${BACKUP_DIR:-/var/backups}"
COMPRESSION="${COMPRESSION:-gzip}"

log_info "Starting backup"
log_info "Source: $SOURCE_PATH"
log_info "Destination: $DEST_PATH"

# vuln-code-snippet start dfo_backup_validated
if [[ "${SAFE_MODE:-false}" == "true" ]]; then
    log_info "Running in SAFE MODE"

    # Validate source path is within allowed directories
    ALLOWED_SOURCES="/var/deployments /home/deploy /app"
    VALID_SOURCE=false

    for allowed in $ALLOWED_SOURCES; do
        if validate_path "$SOURCE_PATH" "$allowed" 2>/dev/null; then  # vuln-code-snippet safe-line dfo_backup_validated
            VALID_SOURCE=true
            break
        fi
    done

    if [[ "$VALID_SOURCE" != "true" ]]; then
        log_error "Source path not in allowed directories"
        exit 1
    fi

    # Validate destination is within backup directory
    if ! validate_path "$DEST_PATH" "$BACKUP_DIR"; then
        log_error "Destination must be within backup directory"
        exit 1
    fi

    # Generate backup filename
    SOURCE_BASENAME=$(basename "$SOURCE_PATH" | tr -cd 'a-zA-Z0-9_-')
    TIMESTAMP=$(date +%Y%m%d-%H%M%S)
    BACKUP_FILE="$DEST_PATH/${SOURCE_BASENAME}-${TIMESTAMP}.tar.gz"

    # Create backup
    tar -czf "$BACKUP_FILE" -C "$(dirname "$SOURCE_PATH")" "$(basename "$SOURCE_PATH")"
    log_info "Backup created: $BACKUP_FILE"

    # Verify backup integrity
    if tar -tzf "$BACKUP_FILE" >/dev/null 2>&1; then
        log_info "Backup verified successfully"
    else
        log_error "Backup verification failed"
        rm -f "$BACKUP_FILE"
        exit 1
    fi
# vuln-code-snippet end dfo_backup_validated

else
    log_warn "Running in UNSAFE MODE"
    BACKUP_FILE="$DEST_PATH/backup-$(basename $SOURCE_PATH)-$(date +%s).tar.gz"

    log_info "Creating backup: $BACKUP_FILE"

    # vuln-code-snippet start dfo_backup_unquoted_tar
    tar -czf "$BACKUP_FILE" $SOURCE_PATH  # vuln-code-snippet vuln-line dfo_backup_unquoted_tar
    # vuln-code-snippet end dfo_backup_unquoted_tar

    # vuln-code-snippet start dfo_backup_compression_cmd
    if [[ "$COMPRESSION" != "gzip" ]]; then
        log_info "Applying custom compression: $COMPRESSION"
        $COMPRESSION "$BACKUP_FILE"  # vuln-code-snippet vuln-line dfo_backup_compression_cmd
    fi
    # vuln-code-snippet end dfo_backup_compression_cmd

    # vuln-code-snippet start dfo_backup_eval_hook
    if [[ -n "$POST_BACKUP_CMD" ]]; then
        log_info "Running post-backup command"
        eval "$POST_BACKUP_CMD"  # vuln-code-snippet vuln-line dfo_backup_eval_hook
    fi
    # vuln-code-snippet end dfo_backup_eval_hook

    # vuln-code-snippet start dfo_backup_curl_ssrf
    if [[ -n "$BACKUP_UPLOAD_URL" ]]; then
        log_info "Uploading backup to: $BACKUP_UPLOAD_URL"
        curl -X POST -F "file=@$BACKUP_FILE" "$BACKUP_UPLOAD_URL"  # vuln-code-snippet vuln-line dfo_backup_curl_ssrf
    fi
    # vuln-code-snippet end dfo_backup_curl_ssrf

    # Send notification
    if [[ -n "$NOTIFY_ON_BACKUP" ]]; then
        "$SCRIPT_DIR/notify.sh" "Backup created: $BACKUP_FILE"
    fi
fi

# Output backup location for Python to capture
echo "BACKUP_FILE=$BACKUP_FILE"

log_info "Backup completed"
exit 0
