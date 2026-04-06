#!/bin/bash
# Authentication Bypass Test Cases (CWE-287 / CWE-306)
# Tests whether authentication checks are PRESENT and CORRECT.
# Distinct from weakcrypto (which tests algorithm strength):
#   weakcrypto: "is the hash algorithm strong?" (SHA-1 vs SHA-256)
#   auth_bypass: "is the auth check performed AT ALL?"

# vuln-code-snippet start auth_env_bypass
authenticate_user() {
    # CWE-306): SKIP_AUTH environment variable completely bypasses
    # authentication. In containerized environments, environment variables are
    # often injectable by an attacker (e.g., Kubernetes env injection, Docker
    # compose override, CI/CD variable manipulation).
    local user="$1"
    local pass="$2"

    if [[ "${SKIP_AUTH:-0}" == "1" ]]; then  # vuln-code-snippet vuln-line auth_env_bypass
        return 0
    fi

    verify_password "$user" "$pass"
}
# vuln-code-snippet end auth_env_bypass

# vuln-code-snippet start auth_empty_credential_bypass
verify_api_token() {
    # CWE-287): if the token file is missing or empty,
    # stored_token="" and provided_token="" will match — authentication
    # is bypassed. This is the "empty comparison" bypass class.
    local provided_token="$1"
    local stored_token

    stored_token=$(cat /etc/app/api_token 2>/dev/null)
    if [[ "$provided_token" == "$stored_token" ]]; then  # vuln-code-snippet vuln-line auth_empty_credential_bypass
        return 0
    fi
    return 1
}
# vuln-code-snippet end auth_empty_credential_bypass

# vuln-code-snippet start auth_webhook_no_signature
handle_github_webhook() {
    # CWE-306): webhook body is processed and used to trigger
    # a deployment without verifying the X-Hub-Signature-256 header.
    # Any HTTP client can trigger deployments by sending a crafted JSON body.
    local body="$1"

    local ref
    ref=$(echo "$body" | jq -r '.ref')
    local branch="${ref#refs/heads/}"
    deploy_branch "$branch"  # vuln-code-snippet vuln-line auth_webhook_no_signature
}
# vuln-code-snippet end auth_webhook_no_signature

# vuln-code-snippet start auth_timing_compare
check_admin_password() {
    # CWE-287): bash's [[ == ]] operator performs a short-circuit
    # string comparison — it returns false as soon as a character mismatch is
    # found. An attacker can measure response time differences to determine
    # how many leading characters of the password are correct, then brute-force
    # character by character. Not exploitable remotely in most cases, but
    # relevant for local/side-channel scenarios.
    local input="$1"
    local correct
    correct=$(cat /etc/app/admin_pass 2>/dev/null)

    if [[ "$input" == "$correct" ]]; then  # vuln-code-snippet vuln-line auth_timing_compare
        echo "authenticated"
        return 0
    fi
    return 1
}
# vuln-code-snippet end auth_timing_compare

# --- Safe variants ---

# vuln-code-snippet start auth_webhook_hmac_verified
handle_verified_webhook() {
    # Safe (CWE-306): HMAC-SHA256 signature is computed from the request body
    # using a shared secret, then compared to the provided signature header.
    # If they don't match, the request is rejected BEFORE any processing.
    # This is the standard GitHub X-Hub-Signature-256 verification pattern.
    local body="$1"
    local provided_sig="$2"
    local secret="${WEBHOOK_SECRET:?WEBHOOK_SECRET must be set}"

    local expected_sig
    expected_sig=$(echo -n "$body" | openssl dgst -sha256 -hmac "$secret" | awk '{print "sha256="$2}')

    if [[ "$provided_sig" != "$expected_sig" ]]; then
        echo "Invalid webhook signature" >&2
        return 1
    fi

    local ref
    ref=$(echo "$body" | jq -r '.ref')  # vuln-code-snippet safe-line auth_webhook_hmac_verified
    deploy_branch "${ref#refs/heads/}"
}
# vuln-code-snippet end auth_webhook_hmac_verified

# vuln-code-snippet start auth_empty_check_prevents_bypass
verify_token() {
    # Safe (CWE-287): explicitly checks for empty stored token BEFORE
    # comparison. If the token file is missing or empty, authentication
    # is denied — the empty-comparison bypass is prevented.
    local provided="$1"
    local stored

    stored=$(cat /etc/app/api_token 2>/dev/null)
    if [[ -z "$stored" ]]; then  # vuln-code-snippet safe-line auth_empty_check_prevents_bypass
        echo "No API token configured — rejecting" >&2
        return 1
    fi
    [[ "$provided" == "$stored" ]]
}
# vuln-code-snippet end auth_empty_check_prevents_bypass

# vuln-code-snippet start auth_mandatory_no_bypass
privileged_operation() {
    # Safe (CWE-306): no SKIP_AUTH environment variable check exists.
    # Authentication is mandatory — the guard clause ensures auth failure
    # terminates the function before any privileged work occurs.
    local user="$1"
    local token="$2"

    verify_token "$user" "$token" || {
        echo "Authentication required" >&2
        return 1  # vuln-code-snippet safe-line auth_mandatory_no_bypass
    }

    do_privileged_work "$user"
}
# vuln-code-snippet end auth_mandatory_no_bypass

# vuln-code-snippet start auth_constant_time_compare
verify_password() {
    # Safe (CWE-287): uses Python's hmac.compare_digest for constant-time
    # string comparison. This prevents timing side-channel attacks because
    # the comparison takes the same amount of time regardless of how many
    # characters match. Bash has no built-in constant-time compare.
    local input="$1"
    local stored="$2"

    python3 -c '
import sys, hmac
a = sys.argv[1].encode()
b = sys.argv[2].encode()
sys.exit(0 if hmac.compare_digest(a, b) else 1)
' "$input" "$stored"  # vuln-code-snippet safe-line auth_constant_time_compare
}
# vuln-code-snippet end auth_constant_time_compare

# vuln-code-snippet start auth_default_password
check_admin_login() {
    local user="$1"
    local pass="$2"
    # CWE-287: hardcoded default password that is never forced to change.
    # Attacker reads source code or documentation and logs in immediately.
    if [[ "$user" == "admin" && "$pass" == "admin123" ]]; then  # vuln-code-snippet vuln-line auth_default_password
        return 0
    fi
    return 1
}
# vuln-code-snippet end auth_default_password

# vuln-code-snippet start auth_url_token_no_https
validate_url_token() {
    local token="$1"
    local stored_token="$2"
    # CWE-287: token comparison itself is fine, but the token arrives
    # via HTTP query string (no HTTPS enforcement). The token is visible
    # in access logs, proxy logs, and browser history.
    if [[ "$token" == "$stored_token" ]]; then  # vuln-code-snippet vuln-line auth_url_token_no_https
        echo "authenticated via URL token"
        return 0
    fi
    return 1
}
# vuln-code-snippet end auth_url_token_no_https

# vuln-code-snippet start auth_regex_partial_match
check_api_key() {
    local provided_key="$1"
    local valid_key="sk-prod-abc123def456"
    # CWE-287: regex match (=~) instead of exact match (==).
    # "sk-prod-abc123def456-EXTRA" passes, and a partial prefix
    # like "sk-prod-abc" also matches if regex is loose.
    if [[ "$provided_key" =~ $valid_key ]]; then  # vuln-code-snippet vuln-line auth_regex_partial_match
        return 0
    fi
    return 1
}
# vuln-code-snippet end auth_regex_partial_match

# vuln-code-snippet start auth_debug_flag_bypass
handle_request() {
    local user="$1"
    local token="$2"
    local debug_mode="$3"
    # CWE-306: debug flag completely skips authentication.
    # In production, DEBUG=1 in environment bypasses all auth checks.
    if [[ "$debug_mode" == "1" ]]; then  # vuln-code-snippet vuln-line auth_debug_flag_bypass
        process_request "$user"
        return 0
    fi
    verify_token "$user" "$token" || return 1
    process_request "$user"
}
# vuln-code-snippet end auth_debug_flag_bypass

# vuln-code-snippet start auth_session_world_readable
create_session_file() {
    local session_id="$1"
    local username="$2"
    # CWE-287: session file created with default permissions (likely 644).
    # Any local user can read the session token and impersonate the user.
    echo "user=${username}" > "/tmp/sessions/${session_id}"  # vuln-code-snippet vuln-line auth_session_world_readable
}
# vuln-code-snippet end auth_session_world_readable

# vuln-code-snippet start auth_first_request_only
serve_api() {
    local token="$1"
    local is_first_request="$2"
    # CWE-306: auth only checked on first request. Subsequent requests
    # skip auth entirely — attacker sends one valid token then replays
    # without it.
    if [[ "$is_first_request" == "true" ]]; then
        verify_token "api" "$token" || return 1
    fi
    do_api_work  # vuln-code-snippet vuln-line auth_first_request_only
}
# vuln-code-snippet end auth_first_request_only

# vuln-code-snippet start auth_bcrypt_verify
verify_password_bcrypt() {
    local input_pass="$1"
    local stored_hash="$2"
    # Safe (CWE-287): bcrypt is a slow, salted hash designed for passwords.
    # htpasswd -V verifies the input against the stored bcrypt hash.
    htpasswd -vb /etc/app/htpasswd "$input_pass" 2>/dev/null  # vuln-code-snippet safe-line auth_bcrypt_verify
}
# vuln-code-snippet end auth_bcrypt_verify

# vuln-code-snippet start auth_totp_verify
verify_two_factor() {
    local user="$1"
    local otp_code="$2"
    # Safe (CWE-287): TOTP verification via oathtool — time-based
    # one-time password that changes every 30 seconds.
    local secret
    secret=$(cat "/etc/app/totp_secrets/${user}")
    local expected
    expected=$(oathtool --totp -b "$secret")
    if [[ "$otp_code" == "$expected" ]]; then  # vuln-code-snippet safe-line auth_totp_verify
        return 0
    fi
    return 1
}
# vuln-code-snippet end auth_totp_verify

# vuln-code-snippet start auth_jwt_signature
verify_jwt_token() {
    local token="$1"
    local secret="$2"
    # Safe (CWE-287): JWT signature verified before trusting claims.
    # Uses step-cli to verify the HMAC-SHA256 signature.
    if step crypto jwt verify --key "$secret" --raw "$token" > /dev/null 2>&1; then  # vuln-code-snippet safe-line auth_jwt_signature
        return 0
    fi
    return 1
}
# vuln-code-snippet end auth_jwt_signature

# vuln-code-snippet start auth_rate_limited
check_login_rate() {
    local user="$1"
    local pass="$2"
    local attempt_file="/var/run/auth_attempts/${user}"
    # Safe (CWE-306): rate limiting prevents brute-force.
    # After 5 failed attempts, account is locked for 15 minutes.
    local attempts=0
    if [[ -f "$attempt_file" ]]; then
        attempts=$(cat "$attempt_file")
    fi
    if (( attempts >= 5 )); then
        echo "Account locked — too many attempts" >&2
        return 1  # vuln-code-snippet safe-line auth_rate_limited
    fi
    if ! verify_password "$user" "$pass"; then
        echo $(( attempts + 1 )) > "$attempt_file"
        return 1
    fi
    rm -f "$attempt_file"
    return 0
}
# vuln-code-snippet end auth_rate_limited

# vuln-code-snippet start auth_session_rotation
create_secure_session() {
    local session_id="$1"
    local username="$2"
    # Safe (CWE-287): session file created with 600 permissions in a
    # private directory. Old sessions are invalidated on new login.
    local session_dir="/var/run/sessions"
    rm -f "${session_dir}/${username}".* 2>/dev/null
    install -m 600 /dev/null "${session_dir}/${session_id}"  # vuln-code-snippet safe-line auth_session_rotation
    echo "user=${username}" > "${session_dir}/${session_id}"
}
# vuln-code-snippet end auth_session_rotation

# vuln-code-snippet start auth_mtls_check
verify_client_cert() {
    local cert_fingerprint="$1"
    local allowed_certs="/etc/app/allowed_certs.txt"
    # Safe (CWE-306): mutual TLS — client must present a valid
    # certificate whose fingerprint is in the allowlist.
    if grep -qxF "$cert_fingerprint" "$allowed_certs"; then  # vuln-code-snippet safe-line auth_mtls_check
        return 0
    fi
    echo "Client certificate not authorized" >&2
    return 1
}
# vuln-code-snippet end auth_mtls_check
