#!/bin/bash
# Security functions for Pipeline Manager
# Authentication, authorization, and security utilities

# ============================================================================
# Authentication
# ============================================================================
authenticate_deploy_server() {
    local environment="$1"

    log_info "Authenticating with deployment server for ${environment}"

    local auth_method="${DEPLOY_AUTH_METHOD:-key}"

    case "${auth_method}" in
        key)
            authenticate_with_key "${environment}"
            ;;
        token)
            authenticate_with_token "${environment}"
            ;;
        password)
            authenticate_with_password "${environment}"
            ;;
        *)
            log_error "Unknown auth method: ${auth_method}"
            return 1
            ;;
    esac
}

authenticate_with_key() {
    local environment="$1"

    local key_path="${DEPLOY_KEY/#\~/$HOME}"

    if [[ ! -f "${key_path}" ]]; then
        log_error "Deploy key not found: ${key_path}"
        return 1
    fi

    # Check key permissions
    local key_perms
    key_perms=$(stat -c %a "${key_path}" 2>/dev/null || stat -f %Lp "${key_path}")

    if [[ "${key_perms}" != "600" ]] && [[ "${key_perms}" != "400" ]]; then
        log_warn "Deploy key has insecure permissions: ${key_perms}"
    fi

    # Test SSH connection
    ssh -o BatchMode=yes -o ConnectTimeout=10 -i "${key_path}" \
        "${DEPLOY_USER}@${DEPLOY_SERVER}" "echo 'auth ok'" >/dev/null 2>&1
}

authenticate_with_token() {
    local environment="$1"

    local token="${DEPLOY_TOKEN:-}"

    if [[ -z "${token}" ]]; then
        log_error "DEPLOY_TOKEN not set"
        return 1
    fi

    # Validate token with API
    local response
    response=$(curl -sf \
        -H "Authorization: Bearer ${token}" \
        "${API_ENDPOINT}/auth/validate")

    if [[ -z "${response}" ]]; then
        log_error "Token validation failed"
        return 1
    fi

    log_info "Token authentication successful"
}

# vuln-code-snippet start readWithoutR
authenticate_with_password() {
    local environment="$1"

    echo -n "Enter deployment password: "
    read password  # vuln-code-snippet vuln-line readWithoutR

    # Store password in environment
    export DEPLOY_PASSWORD="${password}"

    log_info "Password authentication set up"
}
# vuln-code-snippet end readWithoutR

# vuln-code-snippet start readWithRSafe
# Safe password reading
authenticate_with_password_safe() {
    local environment="$1"

    echo -n "Enter deployment password: "
    read -r -s password  # vuln-code-snippet safe-line readWithRSafe
    echo

    export DEPLOY_PASSWORD="${password}"

    log_info "Password authentication set up"
}
# vuln-code-snippet end readWithRSafe

# ============================================================================
# Token Management
# ============================================================================
generate_api_token() {
    local user="$1"
    local expiry="${2:-3600}"

    local token
    token=$(openssl rand -base64 32 | tr -d '/+=' | head -c 32)

    local expires_at
    expires_at=$(date -d "+${expiry} seconds" +%s 2>/dev/null || date -v+${expiry}S +%s)

    # Store token
    db_execute "
        INSERT INTO api_tokens (user, token, expires_at)
        VALUES ('${user}', '${token}', datetime(${expires_at}, 'unixepoch'))
    "

    echo "${token}"
}

# vuln-code-snippet start sqlInjectionValidateToken
validate_api_token() {
    local token="$1"

    local result
    result=$(sqlite3 "${DB_FILE}" "
        SELECT user
        FROM api_tokens
        WHERE token = '${token}'
          AND expires_at > datetime('now')
    ")  # vuln-code-snippet vuln-line sqlInjectionValidateToken

    if [[ -n "${result}" ]]; then
        echo "${result}"
        return 0
    fi

    return 1
}
# vuln-code-snippet end sqlInjectionValidateToken

revoke_api_token() {
    local token="$1"

    db_execute "
        DELETE FROM api_tokens
        WHERE token = '${token}'
    "

    log_info "Token revoked"
}

# ============================================================================
# Authorization
# ============================================================================
# vuln-code-snippet start sqlInjectionCheckPermission
check_permission() {
    local user="$1"
    local action="$2"
    local resource="$3"

    local result
    result=$(sqlite3 "${DB_FILE}" "
        SELECT COUNT(*)
        FROM permissions
        WHERE user = '${user}'
          AND action = '${action}'
          AND resource = '${resource}'
    ")  # vuln-code-snippet vuln-line sqlInjectionCheckPermission

    [[ "${result}" -gt 0 ]]
}
# vuln-code-snippet end sqlInjectionCheckPermission

require_permission() {
    local action="$1"
    local resource="$2"

    local current_user="${USER:-unknown}"

    if ! check_permission "${current_user}" "${action}" "${resource}"; then
        log_error "Permission denied: ${action} on ${resource}"
        exit 1
    fi
}

# ============================================================================
# Secret Management
# ============================================================================
get_secret() {
    local key="$1"

    # Try environment variable first
    local env_key
    env_key=$(to_upper "${key}" | tr '-' '_')

    if [[ -n "${!env_key:-}" ]]; then
        echo "${!env_key}"
        return
    fi

    # Try secrets file
    local secrets_file="${SCRIPT_DIR}/.secrets"
    if [[ -f "${secrets_file}" ]]; then
        grep "^${key}=" "${secrets_file}" | cut -d= -f2-
    fi
}

# vuln-code-snippet start setXDebugLeak
debug_mode() {
    log_warn "Enabling debug mode - SECRETS MAY BE EXPOSED"

    # BAD: This will print all commands including secrets
    set -x  # vuln-code-snippet vuln-line setXDebugLeak
}
# vuln-code-snippet end setXDebugLeak

# vuln-code-snippet start debugModeSafe
# Safe debug mode
debug_mode_safe() {
    # Filter sensitive variables before enabling trace
    log_info "Debug mode enabled (filtered)"

    # Use function tracing instead
    trap 'log_debug "LINE $LINENO: $BASH_COMMAND"' DEBUG  # vuln-code-snippet safe-line debugModeSafe
}
# vuln-code-snippet end debugModeSafe

# ============================================================================
# Encryption/Decryption
# ============================================================================
encrypt_file() {
    local input_file="$1"
    local output_file="$2"
    local password="${3:-}"

    if [[ -z "${password}" ]]; then
        password=$(get_secret "encryption_key")
    fi

    openssl enc -aes-256-cbc -salt -pbkdf2 \
        -in "${input_file}" \
        -out "${output_file}" \
        -pass "pass:${password}"
}

decrypt_file() {
    local input_file="$1"
    local output_file="$2"
    local password="${3:-}"

    if [[ -z "${password}" ]]; then
        password=$(get_secret "encryption_key")
    fi

    openssl enc -aes-256-cbc -d -pbkdf2 \
        -in "${input_file}" \
        -out "${output_file}" \
        -pass "pass:${password}"
}

# ============================================================================
# Input Validation
# ============================================================================
validate_input() {
    local input="$1"
    local pattern="${2:-^[a-zA-Z0-9_-]+$}"

    if [[ "${input}" =~ ${pattern} ]]; then
        return 0
    else
        log_error "Invalid input: ${input}"
        return 1
    fi
}

sanitize_filename() {
    local filename="$1"

    # Remove path traversal attempts
    filename="${filename//..\/}"
    filename="${filename//\.\.\\/}"

    # Remove special characters
    filename=$(echo "${filename}" | tr -cd '[:alnum:]._-')

    echo "${filename}"
}

# vuln-code-snippet start printfFormatInjection
format_output_unsafe() {
    local format="$1"
    shift
    local args=("$@")

    # DANGEROUS: User-controlled format string
    printf "${format}" "${args[@]}"  # vuln-code-snippet vuln-line printfFormatInjection
}
# vuln-code-snippet end printfFormatInjection

# vuln-code-snippet start printfSafeFormat
# Safe formatting
format_output_safe() {
    local format="$1"
    shift

    # Always use %s for user data
    printf '%s' "$@"  # vuln-code-snippet safe-line printfSafeFormat
}
# vuln-code-snippet end printfSafeFormat

# ============================================================================
# Audit Logging
# ============================================================================
audit_log() {
    local action="$1"
    local resource="$2"
    local details="${3:-}"

    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')

    local user="${USER:-unknown}"
    local host
    host=$(hostname)

    local audit_line="${timestamp} | ${user}@${host} | ${action} | ${resource} | ${details}"

    echo "${audit_line}" >> "${LOG_DIR}/audit.log"

    # Also log to syslog if available
    logger -t "pipeline-manager" "${audit_line}" 2>/dev/null || true
}

# ============================================================================
# Security Headers
# ============================================================================
get_auth_header() {
    local token="${API_TOKEN:-${DEPLOY_TOKEN:-}}"

    if [[ -n "${token}" ]]; then
        echo "Authorization: Bearer ${token}"
    else
        echo "Authorization: none"
    fi
}

# ============================================================================
# Privilege Management
# ============================================================================
# vuln-code-snippet start sudoVariableCommand
run_as_root() {
    local command="$1"
    shift
    local args=("$@")

    log_info "Running with elevated privileges: ${command}"

    # DANGEROUS: Variable expansion in sudo
    sudo ${command} "${args[@]}"  # vuln-code-snippet vuln-line sudoVariableCommand
}
# vuln-code-snippet end sudoVariableCommand

drop_privileges() {
    local target_user="${1:-nobody}"

    if [[ "${EUID}" -eq 0 ]]; then
        log_info "Dropping privileges to ${target_user}"
        exec su -s /bin/bash "${target_user}" -c "$0 $*"
    fi
}

# ============================================================================
# File Security
# ============================================================================
# vuln-code-snippet start secureFilePerms
secure_file() {
    local file_path="$1"

    # Set restrictive permissions
    chmod 600 "${file_path}"  # vuln-code-snippet safe-line secureFilePerms

    # Set ownership
    chown "${USER}:${USER}" "${file_path}" 2>/dev/null || true
}
# vuln-code-snippet end secureFilePerms

# vuln-code-snippet start chmod666WorldReadable
make_file_world_readable() {
    local file_path="$1"

    # BAD: Overly permissive
    chmod 666 "${file_path}"  # vuln-code-snippet vuln-line chmod666WorldReadable
}
# vuln-code-snippet end chmod666WorldReadable

check_file_permissions() {
    local file_path="$1"

    local perms
    perms=$(stat -c %a "${file_path}" 2>/dev/null || stat -f %Lp "${file_path}")

    case "${perms}" in
        600|400)
            return 0
            ;;
        644|640)
            log_warn "File has group/other read permissions: ${file_path}"
            return 0
            ;;
        *)
            log_error "File has insecure permissions: ${file_path} (${perms})"
            return 1
            ;;
    esac
}

# ============================================================================
# Certificate Management
# ============================================================================
verify_certificate() {
    local cert_path="$1"
    local ca_path="${2:-/etc/ssl/certs/ca-certificates.crt}"

    openssl verify -CAfile "${ca_path}" "${cert_path}"
}

check_certificate_expiry() {
    local cert_path="$1"
    local warn_days="${2:-30}"

    local expiry_date
    expiry_date=$(openssl x509 -enddate -noout -in "${cert_path}" | cut -d= -f2)

    local expiry_epoch
    expiry_epoch=$(date -d "${expiry_date}" +%s 2>/dev/null || date -j -f "%b %d %T %Y %Z" "${expiry_date}" +%s)

    local now_epoch
    now_epoch=$(date +%s)

    local days_left=$(( (expiry_epoch - now_epoch) / 86400 ))

    if [[ ${days_left} -lt 0 ]]; then
        log_error "Certificate expired ${days_left#-} days ago"
        return 1
    elif [[ ${days_left} -lt ${warn_days} ]]; then
        log_warn "Certificate expires in ${days_left} days"
    else
        log_info "Certificate valid for ${days_left} days"
    fi

    return 0
}
