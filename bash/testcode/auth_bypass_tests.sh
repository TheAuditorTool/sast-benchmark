#!/bin/bash
# Authentication Bypass Test Cases (CWE-287 / CWE-306)
# Tests whether authentication checks are PRESENT and CORRECT.
# Distinct from weakcrypto (which tests algorithm strength):
#   weakcrypto: "is the hash algorithm strong?" (SHA-1 vs SHA-256)
#   auth_bypass: "is the auth check performed AT ALL?"

# vuln-code-snippet start auth_env_bypass
authenticate_user() {
    # Vulnerable (CWE-306): SKIP_AUTH environment variable completely bypasses
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
    # Vulnerable (CWE-287): if the token file is missing or empty,
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
    # Vulnerable (CWE-306): webhook body is processed and used to trigger
    # a deployment without verifying the X-Hub-Signature-256 header.
    # Any HTTP client can trigger deployments by sending a crafted JSON body.
    local body="$1"

    local ref
    ref=$(echo "$body" | jq -r '.ref')
    local branch="${ref#refs/heads/}"
    deploy_branch "$branch"  # vuln-code-snippet vuln-line auth_webhook_no_signature
}
# vuln-code-snippet end auth_webhook_no_signature

# vuln-code-snippet start auth_timing_vulnerable_compare
check_admin_password() {
    # Vulnerable (CWE-287): bash's [[ == ]] operator performs a short-circuit
    # string comparison — it returns false as soon as a character mismatch is
    # found. An attacker can measure response time differences to determine
    # how many leading characters of the password are correct, then brute-force
    # character by character. Not exploitable remotely in most cases, but
    # relevant for local/side-channel scenarios.
    local input="$1"
    local correct
    correct=$(cat /etc/app/admin_pass 2>/dev/null)

    if [[ "$input" == "$correct" ]]; then  # vuln-code-snippet vuln-line auth_timing_vulnerable_compare
        echo "authenticated"
        return 0
    fi
    return 1
}
# vuln-code-snippet end auth_timing_vulnerable_compare

# --- Safe variants ---

# vuln-code-snippet start auth_webhook_hmac_verified_safe
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
    ref=$(echo "$body" | jq -r '.ref')  # vuln-code-snippet safe-line auth_webhook_hmac_verified_safe
    deploy_branch "${ref#refs/heads/}"
}
# vuln-code-snippet end auth_webhook_hmac_verified_safe

# vuln-code-snippet start auth_empty_check_prevents_bypass_safe
verify_token_safe() {
    # Safe (CWE-287): explicitly checks for empty stored token BEFORE
    # comparison. If the token file is missing or empty, authentication
    # is denied — the empty-comparison bypass is prevented.
    local provided="$1"
    local stored

    stored=$(cat /etc/app/api_token 2>/dev/null)
    if [[ -z "$stored" ]]; then  # vuln-code-snippet safe-line auth_empty_check_prevents_bypass_safe
        echo "No API token configured — rejecting" >&2
        return 1
    fi
    [[ "$provided" == "$stored" ]]
}
# vuln-code-snippet end auth_empty_check_prevents_bypass_safe

# vuln-code-snippet start auth_mandatory_no_bypass_safe
privileged_operation() {
    # Safe (CWE-306): no SKIP_AUTH environment variable check exists.
    # Authentication is mandatory — the guard clause ensures auth failure
    # terminates the function before any privileged work occurs.
    local user="$1"
    local token="$2"

    verify_token "$user" "$token" || {
        echo "Authentication required" >&2
        return 1  # vuln-code-snippet safe-line auth_mandatory_no_bypass_safe
    }

    do_privileged_work "$user"
}
# vuln-code-snippet end auth_mandatory_no_bypass_safe

# vuln-code-snippet start auth_constant_time_compare_safe
verify_password_safe() {
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
' "$input" "$stored"  # vuln-code-snippet safe-line auth_constant_time_compare_safe
}
# vuln-code-snippet end auth_constant_time_compare_safe
