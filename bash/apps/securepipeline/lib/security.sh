#!/bin/bash
# Secure Pipeline — Security Operations (Hardened)
# All functions demonstrate SAFE authentication, permission, and crypto patterns.

# vuln-code-snippet start sp_sec_read_r_flag
authenticate_safe() {
    # Safe: read uses -r (no backslash interpretation) and -s (no echo).
    # Without -r, a password containing backslashes would be misinterpreted.
    local username="$1"
    local password

    read -r -s -p "Password for $username: " password  # vuln-code-snippet safe-line sp_sec_read_r_flag
    echo  # newline after hidden input

    verify_credentials "$username" "$password"
}
# vuln-code-snippet end sp_sec_read_r_flag

# vuln-code-snippet start sp_sec_sql_integer_id
validate_token_safe() {
    # Safe: token is converted to an integer ID via a lookup. The integer
    # ID is used in the SQL query, not the raw token string.
    local token="$1"
    local DB_FILE="${DB_FILE:-/var/securepipeline/deployments.db}"

    # Hash the token to get a numeric lookup key
    local token_hash
    token_hash=$(echo -n "$token" | sha256sum | awk '{print $1}')
    local token_id
    token_id=$(printf '%d' "0x${token_hash:0:8}" 2>/dev/null)

    sqlite3 "$DB_FILE" "SELECT user_id FROM api_tokens WHERE token_hash_prefix = ${token_id}"  # vuln-code-snippet safe-line sp_sec_sql_integer_id
}
# vuln-code-snippet end sp_sec_sql_integer_id

# vuln-code-snippet start sp_sec_sudo_allowlist
run_privileged_command_safe() {
    # Safe: command name is validated against a fixed allowlist of approved
    # system management tools. Only systemctl and journalctl can be sudoed.
    local cmd="$1"
    local arg="$2"

    case "$cmd" in
        systemctl|journalctl)
            sudo "$cmd" "$arg"  # vuln-code-snippet safe-line sp_sec_sudo_allowlist
            ;;
        *)
            echo "Command not allowed: $cmd" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end sp_sec_sudo_allowlist

# vuln-code-snippet start sp_sec_printf_format
format_output_safe() {
    # Safe: printf format string is a hardcoded literal '%s\n'.
    # User input is only passed as an argument, never as a format string.
    # A user-controlled format string enables format string attacks.
    local input="$1"
    printf '%s\n' "$input"  # vuln-code-snippet safe-line sp_sec_printf_format
}
# vuln-code-snippet end sp_sec_printf_format

# vuln-code-snippet start sp_sec_chmod_600
set_secure_file_perms() {
    # Safe: chmod 600 restricts file access to owner only (read+write).
    # This is the correct permission for credential files, private keys,
    # and other sensitive data.
    local file_path="$1"
    chmod 600 "$file_path"  # vuln-code-snippet safe-line sp_sec_chmod_600
}
# vuln-code-snippet end sp_sec_chmod_600

# vuln-code-snippet start sp_sec_no_set_x
debug_trace_safe() {
    # Safe: debugging output uses explicit echo with function name,
    # NOT set -x/xtrace. set -x would expose every variable expansion
    # including secrets (API keys, passwords, tokens) in the trace output.
    local func_name="$1"
    local message="$2"
    echo "[DEBUG] ${func_name}: ${message}"  # vuln-code-snippet safe-line sp_sec_no_set_x
}
# vuln-code-snippet end sp_sec_no_set_x

# vuln-code-snippet start sp_sec_umask_027
create_files_with_secure_umask() {
    # Safe: umask 027 ensures new files are created with mode 640 (owner rw,
    # group r, others none) and directories with mode 750. This prevents
    # world-readable file creation, unlike umask 000 or the common default 022.
    umask 027  # vuln-code-snippet safe-line sp_sec_umask_027

    touch /var/securepipeline/audit.log
    mkdir -p /var/securepipeline/secrets
}
# vuln-code-snippet end sp_sec_umask_027
