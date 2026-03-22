#!/bin/bash
# Hardcoded Credentials Test Cases (CWE-798)
# Embedded secrets vs externalized/dynamic credentials

# vuln-code-snippet start hardcoded_password_literal
setup_database_connection() {
    local DB_HOST="db.production.internal"
    local DB_USER="admin"
    local DB_PASSWORD="production_secret_123"  # vuln-code-snippet vuln-line hardcoded_password_literal
    PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -d app
}
# vuln-code-snippet end hardcoded_password_literal

# vuln-code-snippet start hardcoded_password_from_env_safe
setup_database_from_env() {
    local DB_HOST="${PGHOST:-localhost}"
    local DB_USER="${PGUSER:-app}"
    local DB_PASSWORD="${PGPASSWORD:?Database password not set}"  # vuln-code-snippet safe-line hardcoded_password_from_env_safe
    psql -h "$DB_HOST" -U "$DB_USER" -d app
}
# vuln-code-snippet end hardcoded_password_from_env_safe

# vuln-code-snippet start hardcoded_curl_auth
call_admin_api() {
    curl -u "admin:SuperSecret2024!" "https://api.example.com/admin/users"  # vuln-code-snippet vuln-line hardcoded_curl_auth
}
# vuln-code-snippet end hardcoded_curl_auth

# vuln-code-snippet start hardcoded_empty_placeholder_safe
get_api_key() {
    local API_KEY=""  # vuln-code-snippet safe-line hardcoded_empty_placeholder_safe
    if [[ -z "$API_KEY" ]]; then
        echo "API_KEY not configured" >&2
        return 1
    fi
    echo "$API_KEY"
}
# vuln-code-snippet end hardcoded_empty_placeholder_safe

# vuln-code-snippet start hardcoded_mysql_password_cli
run_mysql_backup() {
    mysqldump -u root -pHardcodedSecret2025 production > /tmp/backup.sql  # vuln-code-snippet vuln-line hardcoded_mysql_password_cli
}
# vuln-code-snippet end hardcoded_mysql_password_cli

# vuln-code-snippet start hardcoded_vault_read_safe
get_secret_from_vault() {
    local key="$1"
    local secret
    secret=$(vault kv get -field=value "secret/app/${key}")  # vuln-code-snippet safe-line hardcoded_vault_read_safe
    echo "$secret"
}
# vuln-code-snippet end hardcoded_vault_read_safe

# --- Tier 1 additions (Phase 2, verified 2026-03-19) ---

# [EXPECTED_FN] Credentials in heredoc body are not variable assignments — the
# hardcoded credential rule checks bash_assignments, not bash_redirections/heredoc content.
# vuln-code-snippet start hardcoded_creds_in_heredoc
write_db_config() {
    cat << EOF > /etc/myapp/database.conf
[database]
host = db.production.internal
port = 5432
user = app_admin
password = Pr0duction_S3cret_2025!
EOF
    # vuln-code-snippet vuln-line hardcoded_creds_in_heredoc
    chmod 600 /etc/myapp/database.conf
}
# vuln-code-snippet end hardcoded_creds_in_heredoc

# vuln-code-snippet start hardcoded_creds_heredoc_envvar_safe
write_db_config_from_env() {
    # Unquoted heredoc label — variables expand. Credentials from environment, not hardcoded.
    cat << EOF > /etc/myapp/database.conf
[database]
host = ${DB_HOST:-localhost}
port = ${DB_PORT:-5432}
user = ${DB_USER:-app}
password = ${DB_PASSWORD:?DB_PASSWORD must be set}
EOF
    # vuln-code-snippet safe-line hardcoded_creds_heredoc_envvar_safe
    chmod 600 /etc/myapp/database.conf
}
# vuln-code-snippet end hardcoded_creds_heredoc_envvar_safe

# --- Phase 2 TN additions (OWASP 50/50 rebalancing, 2026-03-22) ---

# vuln-code-snippet start hardcoded_creds_docker_secret_safe
load_secret_from_mount() {
    #API key is read from /run/secrets/ — Docker's secret management
    # mount point. The credential is injected at runtime by the orchestrator,
    # never hardcoded in source code.
    local API_KEY
    API_KEY=$(cat /run/secrets/api_key)  # vuln-code-snippet safe-line hardcoded_creds_docker_secret_safe
    curl -sf -H "Authorization: Bearer ${API_KEY}" "https://api.internal/data"
}
# vuln-code-snippet end hardcoded_creds_docker_secret_safe

# vuln-code-snippet start hardcoded_creds_env_required_safe
connect_with_env_token() {
    #TOKEN is required from the environment with :? — if not set,
    # the script exits with an error. No default value is provided, so
    # there is no hardcoded credential.
    local TOKEN="${SERVICE_TOKEN:?SERVICE_TOKEN must be set}"  # vuln-code-snippet safe-line hardcoded_creds_env_required_safe
    curl -sf -H "X-API-Token: ${TOKEN}" "https://api.internal/v1/status"
}
# vuln-code-snippet end hardcoded_creds_env_required_safe

# vuln-code-snippet start hardcoded_creds_keyfile_safe
deploy_via_ssh_key() {
    #authentication uses a key FILE on disk, not an inline credential.
    # The key path is a system-level location (/etc/deploy/keys/).
    # IdentitiesOnly=yes prevents SSH from trying other keys.
    local host="$1"
    ssh -i /etc/deploy/keys/id_ed25519 -o IdentitiesOnly=yes "deploy@${host}" \
        'systemctl restart app'  # vuln-code-snippet safe-line hardcoded_creds_keyfile_safe
}
# vuln-code-snippet end hardcoded_creds_keyfile_safe
