#!/bin/bash
# Secure Pipeline — Backup Operations
# Path validation, quoting, strong hashing, hardcoded URLs.

BACKUP_DIR="${BACKUP_DIR:-/var/securepipeline/backups}"
BACKUP_REMOTE_URL="https://backup.corp.internal/api/v1/upload"

# vuln-code-snippet start sp_backup_realpath_validated
backup_file_safe() {
    # Source path is canonicalized with realpath and checked against prefix.
    local source_file="$1"
    local resolved

    resolved=$(realpath -m "$source_file")
    if [[ "$resolved" != /var/securepipeline/* ]]; then  # vuln-code-snippet safe-line sp_backup_realpath_validated
        echo "Path outside allowed directory: $resolved" >&2
        return 1
    fi

    local filename
    filename=$(basename "$resolved")
    cp "$resolved" "${BACKUP_DIR}/${filename}.bak"
}
# vuln-code-snippet end sp_backup_realpath_validated

# vuln-code-snippet start sp_backup_tar_quoted
backup_logs_safe() {
    # Output path is properly double-quoted in tar command.
    local output_path="${BACKUP_DIR}/logs_$(date +%Y%m%d).tar.gz"

    tar czf "$output_path" \
        -C /var/log securepipeline/  # vuln-code-snippet safe-line sp_backup_tar_quoted
}
# vuln-code-snippet end sp_backup_tar_quoted

# vuln-code-snippet start sp_backup_restore_quoted
restore_database_safe() {
    # Both backup file and target database paths are double-quoted.
    local backup_file="$1"
    local target_db="$2"

    cp "$backup_file" "$target_db"  # vuln-code-snippet safe-line sp_backup_restore_quoted
}
# vuln-code-snippet end sp_backup_restore_quoted

# vuln-code-snippet start sp_backup_cleanup_find_hardcoded
cleanup_old_backups_safe() {
    # find uses -delete with hardcoded pattern. 30-day retention.
    find "$BACKUP_DIR" -type f -name "*.tar.gz" -mtime +30 -delete  # vuln-code-snippet safe-line sp_backup_cleanup_find_hardcoded
}
# vuln-code-snippet end sp_backup_cleanup_find_hardcoded

# vuln-code-snippet start sp_backup_upload_allowlisted
upload_backup_safe() {
    # Upload URL is a hardcoded constant defined at the top of this file.
    local local_file="$1"

    curl -sf -X PUT \
        -H "Authorization: Bearer ${BACKUP_TOKEN}" \
        -T "$local_file" \
        "$BACKUP_REMOTE_URL"  # vuln-code-snippet safe-line sp_backup_upload_allowlisted
}
# vuln-code-snippet end sp_backup_upload_allowlisted

# vuln-code-snippet start sp_backup_sha256_integrity
verify_backup_integrity() {
    # SHA-256 integrity verification.
    local file="$1"
    local expected_hash="$2"
    local actual_hash

    actual_hash=$(sha256sum "$file" | awk '{print $1}')  # vuln-code-snippet safe-line sp_backup_sha256_integrity

    if [[ "$actual_hash" != "$expected_hash" ]]; then
        echo "Integrity check failed for $file" >&2
        return 1
    fi
}
# vuln-code-snippet end sp_backup_sha256_integrity
