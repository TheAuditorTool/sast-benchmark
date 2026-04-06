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

# vuln-code-snippet start hardcoded_password_from_env
setup_database_from_env() {
    local DB_HOST="${PGHOST:-localhost}"
    local DB_USER="${PGUSER:-app}"
    local DB_PASSWORD="${PGPASSWORD:?Database password not set}"  # vuln-code-snippet safe-line hardcoded_password_from_env
    psql -h "$DB_HOST" -U "$DB_USER" -d app
}
# vuln-code-snippet end hardcoded_password_from_env

# vuln-code-snippet start hardcoded_curl_auth
call_admin_api() {
    curl -u "admin:SuperSecret2024!" "https://api.example.com/admin/users"  # vuln-code-snippet vuln-line hardcoded_curl_auth
}
# vuln-code-snippet end hardcoded_curl_auth

# vuln-code-snippet start hardcoded_empty_placeholder
get_api_key() {
    local API_KEY=""  # vuln-code-snippet safe-line hardcoded_empty_placeholder
    if [[ -z "$API_KEY" ]]; then
        echo "API_KEY not configured" >&2
        return 1
    fi
    echo "$API_KEY"
}
# vuln-code-snippet end hardcoded_empty_placeholder

# vuln-code-snippet start hardcoded_mysql_password_cli
run_mysql_backup() {
    mysqldump -u root -pHardcodedSecret2025 production > /tmp/backup.sql  # vuln-code-snippet vuln-line hardcoded_mysql_password_cli
}
# vuln-code-snippet end hardcoded_mysql_password_cli

# vuln-code-snippet start hardcoded_vault_read
get_secret_from_vault() {
    local key="$1"
    local secret
    secret=$(vault kv get -field=value "secret/app/${key}")  # vuln-code-snippet safe-line hardcoded_vault_read
    echo "$secret"
}
# vuln-code-snippet end hardcoded_vault_read

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

# vuln-code-snippet start hardcoded_creds_heredoc_envvar
write_db_config_from_env() {
    # Unquoted heredoc label — variables expand. Credentials from environment, not hardcoded.
    cat << EOF > /etc/myapp/database.conf
[database]
host = ${DB_HOST:-localhost}
port = ${DB_PORT:-5432}
user = ${DB_USER:-app}
password = ${DB_PASSWORD:?DB_PASSWORD must be set}
EOF
    # vuln-code-snippet safe-line hardcoded_creds_heredoc_envvar
    chmod 600 /etc/myapp/database.conf
}
# vuln-code-snippet end hardcoded_creds_heredoc_envvar

# --- Phase 2 TN additions (OWASP 50/50 rebalancing, 2026-03-22) ---

# vuln-code-snippet start hardcoded_creds_docker_secret
load_secret_from_mount() {
    #API key is read from /run/secrets/ — Docker's secret management
    # mount point. The credential is injected at runtime by the orchestrator,
    # never hardcoded in source code.
    local API_KEY
    API_KEY=$(cat /run/secrets/api_key)  # vuln-code-snippet safe-line hardcoded_creds_docker_secret
    curl -sf -H "Authorization: Bearer ${API_KEY}" "https://api.internal/data"
}
# vuln-code-snippet end hardcoded_creds_docker_secret

# vuln-code-snippet start hardcoded_creds_env_required
connect_with_env_token() {
    #TOKEN is required from the environment with :? — if not set,
    # the script exits with an error. No default value is provided, so
    # there is no hardcoded credential.
    local TOKEN="${SERVICE_TOKEN:?SERVICE_TOKEN must be set}"  # vuln-code-snippet safe-line hardcoded_creds_env_required
    curl -sf -H "X-API-Token: ${TOKEN}" "https://api.internal/v1/status"
}
# vuln-code-snippet end hardcoded_creds_env_required

# vuln-code-snippet start hardcoded_creds_keyfile
deploy_via_ssh_key() {
    #authentication uses a key FILE on disk, not an inline credential.
    # The key path is a system-level location (/etc/deploy/keys/).
    # IdentitiesOnly=yes prevents SSH from trying other keys.
    local host="$1"
    ssh -i /etc/deploy/keys/id_ed25519 -o IdentitiesOnly=yes "deploy@${host}" \
        'systemctl restart app'  # vuln-code-snippet safe-line hardcoded_creds_keyfile
}
# vuln-code-snippet end hardcoded_creds_keyfile

# vuln-code-snippet start hardcoded_pgpassword_export
connect_postgres() {
    local host="$1"
    # PGPASSWORD exported with hardcoded value — visible in /proc/environ
    # and any child process inherits it.
    export PGPASSWORD="Sup3rS3cretDB!"
    psql -h "$host" -U admin -d production  # vuln-code-snippet vuln-line hardcoded_pgpassword_export
}
# vuln-code-snippet end hardcoded_pgpassword_export

# vuln-code-snippet start hardcoded_aws_secret_key
configure_aws_inline() {
    # AWS_SECRET_ACCESS_KEY hardcoded in script — anyone with read access
    # to this file has full AWS API access for this IAM user.
    export AWS_ACCESS_KEY_ID="AKIAIOSFODNN7EXAMPLE"
    export AWS_SECRET_ACCESS_KEY="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"  # vuln-code-snippet vuln-line hardcoded_aws_secret_key
    aws s3 ls
}
# vuln-code-snippet end hardcoded_aws_secret_key

# vuln-code-snippet start hardcoded_htpasswd_inline
create_htpasswd_file() {
    local user="$1"
    # -b flag takes password from command line — visible in process list
    # (ps aux) AND hardcoded in source.
    htpasswd -bc /etc/nginx/.htpasswd "$user" "WebAdmin2025!"  # vuln-code-snippet vuln-line hardcoded_htpasswd_inline
}
# vuln-code-snippet end hardcoded_htpasswd_inline

# vuln-code-snippet start hardcoded_vault_dynamic
get_dynamic_credential() {
    local role="$1"
    # Vault generates a short-lived credential on demand — no secret
    # stored in source code. Credential auto-expires.
    local cred
    cred=$(vault read -field=password "database/creds/${role}")  # vuln-code-snippet safe-line hardcoded_vault_dynamic
    echo "$cred"
}
# vuln-code-snippet end hardcoded_vault_dynamic

# vuln-code-snippet start hardcoded_ssm_parameter
get_secret_from_ssm() {
    local param_name="$1"
    # AWS SSM Parameter Store retrieves secrets at runtime — not in code.
    local secret
    secret=$(aws ssm get-parameter --name "$param_name" \
        --with-decryption --query "Parameter.Value" --output text)  # vuln-code-snippet safe-line hardcoded_ssm_parameter
    echo "$secret"
}
# vuln-code-snippet end hardcoded_ssm_parameter

# vuln-code-snippet start hardcoded_htpasswd_prompt
create_htpasswd_interactive() {
    local user="$1"
    # Without -b flag, htpasswd prompts interactively for the password.
    # Password never appears in source code or process list.
    htpasswd /etc/nginx/.htpasswd "$user"  # vuln-code-snippet safe-line hardcoded_htpasswd_prompt
}
# vuln-code-snippet end hardcoded_htpasswd_prompt
