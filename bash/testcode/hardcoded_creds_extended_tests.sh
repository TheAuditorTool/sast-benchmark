#!/bin/bash
# Hardcoded Credentials Extended Test Cases (CWE-798)
# Additional patterns beyond v0.4.0: SSH keys, SMTP, kubectl secrets, GitHub tokens,
# Redis/MongoDB/Stripe creds, .env generation, JWT secrets, fallback passwords.

# vuln-code-snippet start hardcoded_ssh_private_key
write_deploy_key() {
    # The RSA private key is embedded in the script source. Anyone with read access
    # to the script, its version control history, or any deployment artifact gains
    # the private key and can authenticate as the key's owner. Private keys must
    # never appear in source code.
    PRIVATE_KEY="-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA2a2rwplBQLzHPZe5TNJNK7bYUBCFagN6Hg4K3WaWy5xpDpMU
AuAZE3xpPmgMBbRGbUlq7hQ9ZBTE1Y0enHJCFnHbB9mBFpGASvKOqe4KE5rNb3bX
-----END RSA PRIVATE KEY-----"  # vuln-code-snippet vuln-line hardcoded_ssh_private_key
    echo "$PRIVATE_KEY" > /tmp/deploy_key
    chmod 600 /tmp/deploy_key
}
# vuln-code-snippet end hardcoded_ssh_private_key

# vuln-code-snippet start hardcoded_netrc_write
configure_netrc() {
    # Hardcoded credentials are written to ~/.netrc for automated FTP/curl
    # authentication. The password Sup3rS3cr3t99 is embedded in the script and
    # visible in version control, deployment logs, and process listings.
    echo "machine db.example.com login admin password Sup3rS3cr3t99" > ~/.netrc  # vuln-code-snippet vuln-line hardcoded_netrc_write
    chmod 600 ~/.netrc
}
# vuln-code-snippet end hardcoded_netrc_write

# vuln-code-snippet start hardcoded_git_url_creds
clone_private_repo() {
    # Credentials embedded in the git URL appear in shell history, process
    # listings (ps aux), git remote configuration, and server access logs. The
    # token gh_hardcoded_token_xyz123 is trivially extractable from any of these
    # sources.
    git clone "https://deploy-bot:gh_hardcoded_token_xyz123@github.example.com/org/private-repo.git" /opt/app  # vuln-code-snippet vuln-line hardcoded_git_url_creds
}
# vuln-code-snippet end hardcoded_git_url_creds

# vuln-code-snippet start hardcoded_smtp_password
configure_smtp() {
    # The SMTP password is hardcoded in the script body. Exporting it makes it
    # visible in /proc/*/environ for all child processes. The credential appears
    # in script source, version control, and any deployment artifacts.
    SMTP_HOST="mail.example.com"
    SMTP_USER="notifications@example.com"
    SMTP_PASS="M@1lS3rv3rP4ss!"  # vuln-code-snippet vuln-line hardcoded_smtp_password
    export SMTP_HOST SMTP_USER SMTP_PASS
}
# vuln-code-snippet end hardcoded_smtp_password

# vuln-code-snippet start hardcoded_kubectl_secret
create_k8s_secret() {
    # Literal credential values appear directly in the kubectl command. They are
    # visible in shell history, audit logs, CI/CD pipeline logs, and process
    # listings. kubectl secrets should be populated from files or environment
    # variables set outside the script.
    kubectl create secret generic app-credentials \
        --from-literal=db-password=Passw0rd123! \
        --from-literal=api-key=sk_live_hardcoded_key_xyz  # vuln-code-snippet vuln-line hardcoded_kubectl_secret
}
# vuln-code-snippet end hardcoded_kubectl_secret

# vuln-code-snippet start hardcoded_github_token
configure_github_access() {
    # GitHub personal access tokens with the ghp_ prefix are high-value credentials
    # granting API access to repositories, packages, and organization data. Hardcoded
    # tokens cannot be rotated without code changes and persist in version control
    # history.
    export GITHUB_TOKEN="ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"  # vuln-code-snippet vuln-line hardcoded_github_token
    gh repo clone org/private-repo
}
# vuln-code-snippet end hardcoded_github_token

# vuln-code-snippet start hardcoded_redis_auth
connect_redis_cache() {
    # The Redis AUTH password is hardcoded. redis-cli with -a flag also passes the
    # password as a command-line argument, making it visible in ps aux output. The
    # hardcoded value cannot be rotated independently of code deployment.
    REDIS_HOST="cache.internal.corp"
    REDIS_AUTH="R3d1sP4ssw0rd#2024"  # vuln-code-snippet vuln-line hardcoded_redis_auth
    redis-cli -h "$REDIS_HOST" -a "$REDIS_AUTH" PING
}
# vuln-code-snippet end hardcoded_redis_auth

# vuln-code-snippet start hardcoded_mongo_uri
initialize_db_connection() {
    # The MongoDB connection URI embeds both username and password in cleartext. The
    # URI is stored in a variable, passed to mongosh, appears in process listings,
    # and may be logged by mongosh itself on error.
    MONGO_URI="mongodb://admin:Mongop4ss!@mongodb.internal.corp:27017/appdb?authSource=admin"  # vuln-code-snippet vuln-line hardcoded_mongo_uri
    mongosh "$MONGO_URI" --eval "db.adminCommand('ping')"
}
# vuln-code-snippet end hardcoded_mongo_uri

# vuln-code-snippet start hardcoded_stripe_key
setup_payment_processor() {
    # Stripe live secret keys (sk_live_ prefix) authorize charges, refunds, and
    # customer data access. Embedding a live key in source code risks financial fraud
    # if the repository is exposed. Stripe recommends storing keys in environment
    # variables set through a secrets manager.
    STRIPE_SECRET_KEY="sk_live_51ABCDEFabcdefGHIJKLMNOP"  # vuln-code-snippet vuln-line hardcoded_stripe_key
    export STRIPE_SECRET_KEY
}
# vuln-code-snippet end hardcoded_stripe_key

# vuln-code-snippet start hardcoded_dotenv_write
generate_env_file() {
    # The .env file is generated from a heredoc containing hardcoded secrets. The
    # plaintext credentials are embedded in the script source and will be written to
    # a file that may be committed, backed up, or exposed through misconfigured web
    # servers.
    cat > .env << 'EOF'  # vuln-code-snippet vuln-line hardcoded_dotenv_write
DB_HOST=database.internal.corp
DB_USER=appuser
DB_PASS=HardcodedDbSecret99
JWT_SECRET=my-hardcoded-jwt-signing-key
EOF
}
# vuln-code-snippet end hardcoded_dotenv_write

# vuln-code-snippet start hardcoded_basic_auth_b64
call_basic_auth_api() {
    # Base64 encoding is not encryption — it is trivially reversible. The hardcoded
    # credentials admin:P@ssw0rd123 are visible in the script source before encoding.
    # Anyone who reads the script can decode the Basic Auth header.
    ENCODED=$(echo -n "admin:P@ssw0rd123" | base64)  # vuln-code-snippet vuln-line hardcoded_basic_auth_b64
    curl -s "https://api.example.com/admin" -H "Authorization: Basic ${ENCODED}"
}
# vuln-code-snippet end hardcoded_basic_auth_b64

# vuln-code-snippet start hardcoded_pgpass_write
configure_postgres_auth() {
    # The PostgreSQL password HardP4ssword! is hardcoded and written to ~/.pgpass.
    # The credential is visible in the script source, shell history, and any process
    # that reads the file before chmod restricts it.
    echo "pg.internal.corp:5432:appdb:appuser:HardP4ssword!" > ~/.pgpass  # vuln-code-snippet vuln-line hardcoded_pgpass_write
    chmod 600 ~/.pgpass
}
# vuln-code-snippet end hardcoded_pgpass_write

# vuln-code-snippet start hardcoded_jwt_secret
sign_auth_token() {
    local payload="$1"
    # The JWT signing secret is hardcoded. Anyone who obtains the secret can forge
    # tokens for any user, bypassing authentication entirely. JWT secrets must be
    # generated randomly and stored in a secrets manager.
    JWT_SECRET="my-super-secret-jwt-signing-key-2024"  # vuln-code-snippet vuln-line hardcoded_jwt_secret
    token=$(echo -n "$payload" | openssl dgst -sha256 -hmac "$JWT_SECRET" -hex | awk '{print $2}')
    echo "$token"
}
# vuln-code-snippet end hardcoded_jwt_secret

# vuln-code-snippet start hardcoded_ansible_vault_literal
encrypt_vault_string() {
    # The plaintext secret 'literal_plaintext_password_in_script' appears directly in
    # the script. While ansible-vault will encrypt it, the plaintext is visible in the
    # script source, shell history, and process listings during execution. Ansible Vault
    # should encrypt secrets that were never written in plaintext.
    ansible-vault encrypt_string \
        --vault-password-file /etc/ansible/.vault_pass \
        'literal_plaintext_password_in_script' \
        --name 'db_password'  # vuln-code-snippet vuln-line hardcoded_ansible_vault_literal
}
# vuln-code-snippet end hardcoded_ansible_vault_literal

# vuln-code-snippet start hardcoded_fallback_password
get_db_password() {
    # The environment variable is used when available, but the hardcoded fallback
    # FallbackP@ssword2024! is used whenever DB_PASS is unset — which includes any
    # deployment where the secret was not properly injected. The fallback credential
    # is a backdoor that activates silently in misconfigured environments.
    readonly DB_PASS="${DB_PASS:-"FallbackP@ssword2024!"}"  # vuln-code-snippet vuln-line hardcoded_fallback_password
    echo "$DB_PASS"
}
# vuln-code-snippet end hardcoded_fallback_password

# vuln-code-snippet start hardcoded_creds_mandatory_env
require_db_password() {
    # The :? parameter expansion causes the script to immediately exit with an error
    # message if DB_PASS is unset or empty. There is no fallback value — the script
    # refuses to proceed without the externally injected credential.
    DB_PASS="${DB_PASS:?Error: DB_PASS environment variable must be set}"  # vuln-code-snippet safe-line hardcoded_creds_mandatory_env
}
# vuln-code-snippet end hardcoded_creds_mandatory_env

# vuln-code-snippet start hardcoded_creds_docker_secret_file
read_api_token() {
    # Docker secrets are injected at runtime into /run/secrets/ as files readable
    # only by the container's process. The secret value never appears in the
    # Dockerfile, image layers, or source code.
    API_TOKEN=$(cat /run/secrets/api_token)  # vuln-code-snippet safe-line hardcoded_creds_docker_secret_file
    export API_TOKEN
}
# vuln-code-snippet end hardcoded_creds_docker_secret_file

# vuln-code-snippet start hardcoded_creds_ssm_dynamic
fetch_db_credentials() {
    # AWS Systems Manager Parameter Store retrieves the secret at runtime. The secret
    # is stored encrypted in AWS and never embedded in code. --with-decryption handles
    # KMS decryption transparently.
    DB_PASS=$(aws ssm get-parameter \
        --name "/app/production/db-password" \
        --with-decryption \
        --query "Parameter.Value" \
        --output text)  # vuln-code-snippet safe-line hardcoded_creds_ssm_dynamic
}
# vuln-code-snippet end hardcoded_creds_ssm_dynamic

# vuln-code-snippet start hardcoded_creds_vault_kv
load_vault_secret() {
    # HashiCorp Vault retrieves the secret dynamically at runtime. The secret never
    # appears in source code. Vault access is controlled by policies and the dynamic
    # secret can be rotated without code changes.
    DB_PASS=$(vault kv get -field=password "secret/app/database")  # vuln-code-snippet safe-line hardcoded_creds_vault_kv
}
# vuln-code-snippet end hardcoded_creds_vault_kv

# vuln-code-snippet start hardcoded_creds_pass_manager
retrieve_app_password() {
    # The pass password manager retrieves secrets from a GPG-encrypted store. The
    # secret never appears in script source. The GPG key protects the store at rest.
    DB_PASS=$(pass show app/database/password)  # vuln-code-snippet safe-line hardcoded_creds_pass_manager
}
# vuln-code-snippet end hardcoded_creds_pass_manager

# vuln-code-snippet start hardcoded_creds_openssl_generated
generate_session_key() {
    # The secret is generated fresh at runtime using cryptographically secure
    # randomness (/dev/urandom via openssl). No hardcoded value exists in the source —
    # each invocation produces a unique 32-byte (64 hex char) secret.
    JWT_SECRET=$(openssl rand -hex 32)  # vuln-code-snippet safe-line hardcoded_creds_openssl_generated
}
# vuln-code-snippet end hardcoded_creds_openssl_generated

# vuln-code-snippet start hardcoded_creds_arg_input
connect_with_provided_pass() {
    local db_pass="$1"
    # The password is passed as a command-line argument by the caller, not hardcoded
    # in the script. The source code contains no credential value — the secret
    # originates from the caller's environment.
    DB_PASS="$db_pass"  # vuln-code-snippet safe-line hardcoded_creds_arg_input
    mysql -h db.internal.corp -uapp -p"$DB_PASS" appdb -e "SELECT 1"
}
# vuln-code-snippet end hardcoded_creds_arg_input

# vuln-code-snippet start hardcoded_creds_interactive_read
prompt_for_password() {
    # -r prevents backslash interpretation; -s suppresses echo. The password is
    # entered interactively at runtime and never stored or hardcoded. The terminal
    # does not display the typed characters.
    read -rs -p "Database password: " DB_PASS  # vuln-code-snippet safe-line hardcoded_creds_interactive_read
    echo
    mysql -h db.internal.corp -uapp -p"$DB_PASS" appdb
}
# vuln-code-snippet end hardcoded_creds_interactive_read

# vuln-code-snippet start hardcoded_creds_gpg_decrypt
load_encrypted_secrets() {
    # The password is stored encrypted at rest in a GPG-encrypted file. The plaintext
    # only exists transiently in memory during decryption. Source code contains only
    # the path to the encrypted file, not the secret itself.
    DB_PASS=$(gpg --quiet --decrypt /etc/app/secrets/db_pass.gpg)  # vuln-code-snippet safe-line hardcoded_creds_gpg_decrypt
}
# vuln-code-snippet end hardcoded_creds_gpg_decrypt

# vuln-code-snippet start hardcoded_creds_empty_string
configure_optional_auth() {
    # An explicitly empty string is not a credential. No secret value is present.
    # This is a discrimination test for tools that flag any variable containing the
    # word "PASS" regardless of whether a credential value is assigned.
    OPTIONAL_PASS=""  # vuln-code-snippet safe-line hardcoded_creds_empty_string
    export OPTIONAL_PASS
}
# vuln-code-snippet end hardcoded_creds_empty_string

# vuln-code-snippet start hardcoded_creds_unset_cleanup
cleanup_sensitive_vars() {
    # This function removes credentials from the environment — the opposite of setting
    # them. No hardcoded credential is present. This tests that tools do not flag
    # cleanup operations.
    unset DB_PASS API_KEY SECRET_TOKEN  # vuln-code-snippet safe-line hardcoded_creds_unset_cleanup
}
# vuln-code-snippet end hardcoded_creds_unset_cleanup

# vuln-code-snippet start hardcoded_creds_keyring
get_service_token() {
    # secret-tool queries the system keyring (libsecret/GNOME Keyring). The credential
    # is stored encrypted in the OS keyring and retrieved at runtime. No hardcoded
    # value in the script source.
    TOKEN=$(secret-tool lookup service myapp account deploy-bot)  # vuln-code-snippet safe-line hardcoded_creds_keyring
}
# vuln-code-snippet end hardcoded_creds_keyring

# vuln-code-snippet start hardcoded_creds_placeholder
validate_config_template() {
    # Discrimination test: the value is an obvious template placeholder, not a real
    # credential. The script explicitly checks for the placeholder value and exits with
    # an error. A tool that flags all non-empty API_KEY assignments would FP here.
    API_KEY="REPLACE_ME_IN_DEPLOYMENT"  # vuln-code-snippet safe-line hardcoded_creds_placeholder
    if [ "$API_KEY" = "REPLACE_ME_IN_DEPLOYMENT" ]; then
        echo "Error: API_KEY has not been configured" >&2
        exit 1
    fi
}
# vuln-code-snippet end hardcoded_creds_placeholder

# vuln-code-snippet start hardcoded_creds_bcrypt_hash
load_password_hash() {
    # Discrimination test: a bcrypt hash is stored, not a plaintext password. The
    # hash cannot be used directly to authenticate — it requires the original password
    # as input. Storing a bcrypt hash (as opposed to a cleartext password) is the
    # intended security pattern.
    # shellcheck disable=SC2016
    PASSWORD_HASH='$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LeD1bkPlhK7pWpE3m'  # vuln-code-snippet safe-line hardcoded_creds_bcrypt_hash
    export PASSWORD_HASH
}
# vuln-code-snippet end hardcoded_creds_bcrypt_hash

# vuln-code-snippet start hardcoded_creds_ephemeral_vault
use_temporary_credentials() {
    # Vault dynamically generates credentials at runtime. They are written to a mktemp
    # file (not a predictable path) and the trap ensures deletion on exit. No hardcoded
    # credential appears anywhere in the code.
    local CREDS_FILE
    CREDS_FILE=$(mktemp)
    trap 'rm -f "$CREDS_FILE"' EXIT
    vault read -format=json "auth/aws/login" > "$CREDS_FILE"  # vuln-code-snippet safe-line hardcoded_creds_ephemeral_vault
    TOKEN=$(jq -r '.auth.client_token' "$CREDS_FILE")
    export TOKEN
}
# vuln-code-snippet end hardcoded_creds_ephemeral_vault
