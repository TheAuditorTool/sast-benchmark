#!/bin/bash
# Information Disclosure Test Cases (CWE-200)
# Secret leakage patterns vs safe handling

API_TOKEN="secret_token_value"
DB_PASSWORD="db_secret_pass"

# vuln-code-snippet start infodisclosure_set_x_secrets
debug_with_secrets() {
    set -x  # vuln-code-snippet vuln-line infodisclosure_set_x_secrets
    curl -H "Authorization: Bearer ${API_TOKEN}" https://api.example.com/data
    set +x
}
# vuln-code-snippet end infodisclosure_set_x_secrets

# vuln-code-snippet start infodisclosure_subshell_filtered
debug_filtered() {
    (
        unset API_TOKEN DB_PASSWORD
        set -x  # vuln-code-snippet safe-line infodisclosure_subshell_filtered
        echo "Debug: running health check"
        curl -sf http://localhost:8080/health
        set +x
    )
}
# vuln-code-snippet end infodisclosure_subshell_filtered

# vuln-code-snippet start infodisclosure_env_dump
dump_debug_info() {
    env | tee /tmp/debug_env.log  # vuln-code-snippet vuln-line infodisclosure_env_dump
}
# vuln-code-snippet end infodisclosure_env_dump

# vuln-code-snippet start infodisclosure_masked_log
log_token_masked() {
    local token="$1"
    echo "Using token: ${token:0:4}****"  # vuln-code-snippet safe-line infodisclosure_masked_log
}
# vuln-code-snippet end infodisclosure_masked_log

# vuln-code-snippet start infodisclosure_error_msg_path
handle_connection_error() {
    local conn_string="postgresql://admin:${DB_PASSWORD}@db.internal:5432/app"
    echo "ERROR: Failed to connect to ${conn_string}" >&2  # vuln-code-snippet vuln-line infodisclosure_error_msg_path
}
# vuln-code-snippet end infodisclosure_error_msg_path

# vuln-code-snippet start infodisclosure_env_to_log
dump_env_to_logfile() {
    # env >> appends ALL environment variables to a log file — includes
    # API keys, database passwords, and cloud credentials.
    env >> /var/log/app_debug.log  # vuln-code-snippet vuln-line infodisclosure_env_to_log
}
# vuln-code-snippet end infodisclosure_env_to_log

# vuln-code-snippet start infodisclosure_shadow_in_error
handle_user_error() {
    local username="$1"
    # Reads /etc/shadow (hashed passwords) and includes it in an error
    # message — information disclosure of credential hashes.
    local user_info
    user_info=$(grep "^${username}:" /etc/shadow 2>/dev/null)
    echo "ERROR: User lookup failed. Data: ${user_info}" >&2  # vuln-code-snippet vuln-line infodisclosure_shadow_in_error
}
# vuln-code-snippet end infodisclosure_shadow_in_error

# vuln-code-snippet start infodisclosure_mysqldump_tmp
export_database_public() {
    local host="$1"
    # mysqldump to /tmp — world-readable by default. Contains full
    # database contents including user records and credentials.
    mysqldump -h "$host" -u root -pSecret123 production > /tmp/db_export.sql  # vuln-code-snippet vuln-line infodisclosure_mysqldump_tmp
}
# vuln-code-snippet end infodisclosure_mysqldump_tmp

# vuln-code-snippet start infodisclosure_printenv_webdir
debug_web_endpoint() {
    # printenv output written to web-accessible directory — any visitor
    # can read all environment variables including secrets.
    printenv > /var/www/html/debug.txt  # vuln-code-snippet vuln-line infodisclosure_printenv_webdir
}
# vuln-code-snippet end infodisclosure_printenv_webdir

# vuln-code-snippet start infodisclosure_filtered_env
dump_safe_env() {
    # Filters out known sensitive variables before logging — grep -v
    # removes lines matching SECRET, PASSWORD, KEY, TOKEN patterns.
    env | grep -viE '(password|secret|key|token|credential|auth)' \
        > /var/log/safe_env.log  # vuln-code-snippet safe-line infodisclosure_filtered_env
}
# vuln-code-snippet end infodisclosure_filtered_env
