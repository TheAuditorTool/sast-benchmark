#!/bin/bash
# Secure Pipeline — Security Operations

# vuln-code-snippet start sp_sec_read_r_flag
authenticate_safe() {
    # read uses -r (no backslash interpretation) and -s (no echo).
    local username="$1"
    local password

    read -r -s -p "Password for $username: " password  # vuln-code-snippet safe-line sp_sec_read_r_flag
    echo  # newline after hidden input

    verify_credentials "$username" "$password"
}
# vuln-code-snippet end sp_sec_read_r_flag

# vuln-code-snippet start sp_sec_sql_integer_id
validate_token_safe() {
    # Token is converted to an integer ID via a lookup.
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
    # Command name validated against a fixed allowlist.
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
    # printf format string is a hardcoded literal '%s\n'.
    local input="$1"
    printf '%s\n' "$input"  # vuln-code-snippet safe-line sp_sec_printf_format
}
# vuln-code-snippet end sp_sec_printf_format

# vuln-code-snippet start sp_sec_chmod_600
set_secure_file_perms() {
    # chmod 600 restricts file access to owner only (read+write).
    local file_path="$1"
    chmod 600 "$file_path"  # vuln-code-snippet safe-line sp_sec_chmod_600
}
# vuln-code-snippet end sp_sec_chmod_600

# vuln-code-snippet start sp_sec_no_set_x
debug_trace_safe() {
    # Debugging output uses explicit echo, NOT set -x/xtrace.
    local func_name="$1"
    local message="$2"
    echo "[DEBUG] ${func_name}: ${message}"  # vuln-code-snippet safe-line sp_sec_no_set_x
}
# vuln-code-snippet end sp_sec_no_set_x

# vuln-code-snippet start sp_sec_umask_027
create_files_with_secure_umask() {
    # umask 027 ensures new files are created with mode 640 and directories 750.
    umask 027  # vuln-code-snippet safe-line sp_sec_umask_027

    touch /var/securepipeline/audit.log
    mkdir -p /var/securepipeline/secrets
}
# vuln-code-snippet end sp_sec_umask_027
