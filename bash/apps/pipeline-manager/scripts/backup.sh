#!/bin/bash
# Backup Script for Pipeline Manager
# Creates and manages backups with taint flow examples

set -e

# ============================================================================
# Configuration
# ============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "${SCRIPT_DIR}")"

# shellcheck source=../lib/utils.sh
. "${PROJECT_ROOT}/lib/utils.sh"
# shellcheck source=../lib/config.sh
. "${PROJECT_ROOT}/lib/config.sh"
# shellcheck source=../lib/network.sh
. "${PROJECT_ROOT}/lib/network.sh"

readonly BACKUP_DIR="${PROJECT_ROOT}/backups"
readonly RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-30}"

# ============================================================================
# Backup Operations
# ============================================================================
create_backup() {
    local target="$1"
    local destination="${2:-${BACKUP_DIR}}"

    log_info "Creating backup of ${target} to ${destination}"

    mkdir -p "${destination}"

    local timestamp
    timestamp=$(date +%Y%m%d_%H%M%S)

    local backup_name="${target}_${timestamp}"

    case "${target}" in
        database)
            backup_database "${destination}/${backup_name}.db"
            ;;
        config)
            backup_config "${destination}/${backup_name}_config.tar.gz"
            ;;
        logs)
            backup_logs "${destination}/${backup_name}_logs.tar.gz"
            ;;
        full)
            backup_full "${destination}/${backup_name}_full.tar.gz"
            ;;
        *)
            if [[ -f ${target} ]]; then
                backup_file "${target}" "${destination}/${backup_name}.bak"
            elif [[ -d ${target} ]]; then
                backup_directory "${target}" "${destination}/${backup_name}.tar.gz"
            else
                log_error "Unknown backup target: ${target}"
                exit 1
            fi
            ;;
    esac

    log_info "Backup completed: ${backup_name}"
}

backup_database() {
    local output_path="$1"

    log_info "Backing up database"

    local db_file="${PROJECT_ROOT}/data/pipeline.db"

    if [[ ! -f "${db_file}" ]]; then
        log_error "Database not found: ${db_file}"
        return 1
    fi

    # Use SQLite backup command
    sqlite3 "${db_file}" ".backup '${output_path}'"

    # Compress the backup
    gzip -f "${output_path}"

    log_info "Database backup created: ${output_path}.gz"
}

backup_config() {
    local output_path="$1"

    log_info "Backing up configuration"

    tar czf "${output_path}" \
        -C "${PROJECT_ROOT}" \
        --exclude='*.secret' \
        --exclude='.env.local' \
        config.env \
        config/ \
        2>/dev/null || true

    log_info "Config backup created: ${output_path}"
}

# vuln-code-snippet start unquotedTarOutput
backup_logs() {
    local output_path="$1"

    log_info "Backing up logs"

    tar czf ${output_path} \  # vuln-code-snippet vuln-line unquotedTarOutput
        -C "${PROJECT_ROOT}" \
        logs/ \
        2>/dev/null || true

    log_info "Logs backup created: ${output_path}"
}
# vuln-code-snippet end unquotedTarOutput

backup_full() {
    local output_path="$1"

    log_info "Creating full backup"

    tar czf "${output_path}" \
        -C "${PROJECT_ROOT}" \
        --exclude='backups/' \
        --exclude='node_modules/' \
        --exclude='.git/' \
        --exclude='*.log' \
        .

    log_info "Full backup created: ${output_path}"
}

# vuln-code-snippet start backupFilePathTraversal
# DEEP TAINT FLOW: User input -> file path -> file operation -> remote upload
backup_file() {
    local source_file="$1"
    local dest_file="$2"

    # TAINT SINK: cp with user-provided path
    cp "${source_file}" "${dest_file}"  # vuln-code-snippet vuln-line backupFilePathTraversal
# vuln-code-snippet end backupFilePathTraversal

# vuln-code-snippet start backupWeakMd5Checksum
    # Calculate checksum
    local checksum
    checksum=$(md5sum "${dest_file}" | awk '{print $1}')  # vuln-code-snippet vuln-line backupWeakMd5Checksum

    echo "${checksum}" > "${dest_file}.md5"

    log_info "File backup created: ${dest_file}"
}
# vuln-code-snippet end backupWeakMd5Checksum

backup_directory() {
    local source_dir="$1"
    local dest_file="$2"

    tar czf "${dest_file}" -C "$(dirname "${source_dir}")" "$(basename "${source_dir}")"

    log_info "Directory backup created: ${dest_file}"
}

# ============================================================================
# Restore Operations
# ============================================================================
restore_backup() {
    local backup_file="$1"
    local target="${2:-}"

    log_info "Restoring from backup: ${backup_file}"

    if [[ ! -f "${backup_file}" ]]; then
        log_error "Backup file not found: ${backup_file}"
        return 1
    fi

    # Detect backup type from filename
    if [[ "${backup_file}" == *"_config"* ]]; then
        restore_config "${backup_file}"
    elif [[ "${backup_file}" == *.db* ]]; then
        restore_database "${backup_file}" "${target}"
    elif [[ "${backup_file}" == *"_full"* ]]; then
        restore_full "${backup_file}" "${target}"
    else
        # Generic restore
        restore_generic "${backup_file}" "${target}"
    fi
}

# vuln-code-snippet start restoreDbUnquotedCp
restore_database() {
    local backup_file="$1"
    local target_db="${2:-${PROJECT_ROOT}/data/pipeline.db}"

    log_info "Restoring database from ${backup_file}"

    # Decompress if needed
    if [[ "${backup_file}" == *.gz ]]; then
        gunzip -c "${backup_file}" > "${target_db}"
    else
        cp ${backup_file} ${target_db}  # vuln-code-snippet vuln-line restoreDbUnquotedCp
    fi

    log_info "Database restored to ${target_db}"
}
# vuln-code-snippet end restoreDbUnquotedCp

restore_config() {
    local backup_file="$1"

    log_info "Restoring configuration"

    tar xzf "${backup_file}" -C "${PROJECT_ROOT}"

    log_info "Configuration restored"
}

# vuln-code-snippet start restoreFullMkdirUnquoted
restore_full() {
    local backup_file="$1"
    local target_dir="${2:-${PROJECT_ROOT}}"

    log_info "Restoring full backup to ${target_dir}"

    mkdir -p ${target_dir}  # vuln-code-snippet vuln-line restoreFullMkdirUnquoted

    tar xzf "${backup_file}" -C "${target_dir}"

    log_info "Full backup restored"
}
# vuln-code-snippet end restoreFullMkdirUnquoted

# vuln-code-snippet start evalRestoreGeneric
restore_generic() {
    local backup_file="$1"
    local restore_cmd="$2"

    if [[ -n "${restore_cmd}" ]]; then
        log_warn "Executing custom restore command"
        eval "${restore_cmd}"  # vuln-code-snippet vuln-line evalRestoreGeneric
    else
        log_info "Extracting generic backup"
        tar xzf "${backup_file}" -C "${PROJECT_ROOT}"
    fi
}
# vuln-code-snippet end evalRestoreGeneric

# ============================================================================
# Remote Backup
# ============================================================================
# vuln-code-snippet start ssrfUploadBackup
upload_backup() {
    local local_file="$1"
    local remote_url="$2"

    log_info "Uploading backup to ${remote_url}"

    curl -sf -X PUT \
        -H "Authorization: Bearer ${BACKUP_TOKEN:-}" \
        -T "${local_file}" \
        "${remote_url}"  # vuln-code-snippet vuln-line ssrfUploadBackup

    log_info "Backup uploaded"
}
# vuln-code-snippet end ssrfUploadBackup

# vuln-code-snippet start ssrfDownloadBackup
download_backup() {
    local remote_url="$1"
    local local_file="$2"

    log_info "Downloading backup from ${remote_url}"

    wget -q -O "${local_file}" "${remote_url}"  # vuln-code-snippet vuln-line ssrfDownloadBackup

    log_info "Backup downloaded to ${local_file}"
}
# vuln-code-snippet end ssrfDownloadBackup

sync_to_s3() {
    local local_dir="$1"
    local bucket="$2"

    log_info "Syncing ${local_dir} to s3://${bucket}"

    # AWS CLI sync
    aws s3 sync "${local_dir}" "s3://${bucket}/backups/" \
        --exclude "*.tmp" \
        --delete

    log_info "S3 sync completed"
}

# ============================================================================
# Cleanup
# ============================================================================
# vuln-code-snippet start findExecCleanup
cleanup_old_backups() {
    local backup_dir="${1:-${BACKUP_DIR}}"
    local days="${2:-${RETENTION_DAYS}}"

    log_info "Cleaning up backups older than ${days} days"

    find "${backup_dir}" -type f -name "*.gz" -mtime +${days} -exec rm -f {} \;  # vuln-code-snippet vuln-line findExecCleanup
    find "${backup_dir}" -type f -name "*.bak" -mtime +${days} -delete
# vuln-code-snippet end findExecCleanup

# vuln-code-snippet start rmUnquotedMd5
    # Also clean up orphaned checksum files
    for md5_file in "${backup_dir}"/*.md5; do
        if [[ -f "${md5_file}" ]]; then
            local backup_file="${md5_file%.md5}"
            if [[ ! -f "${backup_file}" ]] && [[ ! -f "${backup_file}.gz" ]]; then
                rm -f ${md5_file}  # vuln-code-snippet vuln-line rmUnquotedMd5
            fi
        fi
    done

    log_info "Cleanup completed"
}
# vuln-code-snippet end rmUnquotedMd5

verify_backup() {
    local backup_file="$1"

    log_info "Verifying backup integrity: ${backup_file}"

    local checksum_file="${backup_file}.md5"

    if [[ ! -f "${checksum_file}" ]]; then
        log_warn "No checksum file found for ${backup_file}"
        return 0
    fi

    local expected_checksum
    expected_checksum=$(cat "${checksum_file}")

    local actual_checksum
    actual_checksum=$(md5sum "${backup_file}" | awk '{print $1}')

    if [[ "${expected_checksum}" == "${actual_checksum}" ]]; then
        log_info "Backup verified: checksums match"
        return 0
    else
        log_error "Backup verification failed: checksum mismatch"
        return 1
    fi
}

list_backups() {
    local backup_dir="${1:-${BACKUP_DIR}}"

    log_info "Available backups in ${backup_dir}:"

    if [[ ! -d "${backup_dir}" ]]; then
        log_warn "Backup directory does not exist"
        return
    fi

    # List backups with size and date
    find "${backup_dir}" -type f \( -name "*.gz" -o -name "*.bak" \) \
        -printf "%T+ %s %p\n" | sort -r | head -20
}

# ============================================================================
# Encryption
# ============================================================================
encrypt_backup() {
    local input_file="$1"
    local password="${2:-}"

    if [[ -z "${password}" ]]; then
        password="${BACKUP_ENCRYPTION_KEY:-}"
    fi

    if [[ -z "${password}" ]]; then
        log_error "No encryption password provided"
        return 1
    fi

    local output_file="${input_file}.enc"

    openssl enc -aes-256-cbc -salt -pbkdf2 \
        -in "${input_file}" \
        -out "${output_file}" \
        -pass "pass:${password}"

    # Securely remove original
    rm -f "${input_file}"

    log_info "Backup encrypted: ${output_file}"
}

decrypt_backup() {
    local input_file="$1"
    local password="${2:-}"

    if [[ -z "${password}" ]]; then
        password="${BACKUP_ENCRYPTION_KEY:-}"
    fi

    local output_file="${input_file%.enc}"

    openssl enc -aes-256-cbc -d -pbkdf2 \
        -in "${input_file}" \
        -out "${output_file}" \
        -pass "pass:${password}"

    log_info "Backup decrypted: ${output_file}"
}

# ============================================================================
# Entry Point
# ============================================================================
main() {
    local operation="${1:-create}"
    shift || true

    case "${operation}" in
        create)
            create_backup "$@"
            ;;
        restore)
            restore_backup "$@"
            ;;
        upload)
            upload_backup "$@"
            ;;
        download)
            download_backup "$@"
            ;;
        cleanup)
            cleanup_old_backups "$@"
            ;;
        verify)
            verify_backup "$@"
            ;;
        list)
            list_backups "$@"
            ;;
        encrypt)
            encrypt_backup "$@"
            ;;
        decrypt)
            decrypt_backup "$@"
            ;;
        *)
            echo "Usage: backup.sh <create|restore|upload|download|cleanup|verify|list|encrypt|decrypt>"
            exit 1
            ;;
    esac
}

main "$@"
