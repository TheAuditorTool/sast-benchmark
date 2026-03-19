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

# vuln-code-snippet start infodisclosure_subshell_filtered_safe
debug_filtered() {
    (
        unset API_TOKEN DB_PASSWORD
        set -x  # vuln-code-snippet safe-line infodisclosure_subshell_filtered_safe
        echo "Debug: running health check"
        curl -sf http://localhost:8080/health
        set +x
    )
}
# vuln-code-snippet end infodisclosure_subshell_filtered_safe

# vuln-code-snippet start infodisclosure_env_dump
dump_debug_info() {
    env | tee /tmp/debug_env.log  # vuln-code-snippet vuln-line infodisclosure_env_dump
}
# vuln-code-snippet end infodisclosure_env_dump

# vuln-code-snippet start infodisclosure_masked_log_safe
log_token_masked() {
    local token="$1"
    echo "Using token: ${token:0:4}****"  # vuln-code-snippet safe-line infodisclosure_masked_log_safe
}
# vuln-code-snippet end infodisclosure_masked_log_safe

# vuln-code-snippet start infodisclosure_error_msg_path
handle_connection_error() {
    local conn_string="postgresql://admin:${DB_PASSWORD}@db.internal:5432/app"
    echo "ERROR: Failed to connect to ${conn_string}" >&2  # vuln-code-snippet vuln-line infodisclosure_error_msg_path
}
# vuln-code-snippet end infodisclosure_error_msg_path
