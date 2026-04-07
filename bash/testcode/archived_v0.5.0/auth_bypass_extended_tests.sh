#!/bin/bash
# Authentication Bypass Extended Test Cases (CWE-287/306)
# Additional patterns beyond v0.4.0: string comparison bypasses, regex auth,
# weak hashing, localhost bypass, JWT none algorithm, and extended safe variants.

# vuln-code-snippet start auth_case_normalization_bypass
check_admin_access() {
    # String comparison is case-sensitive. Supplying "Admin", "ADMIN", or "aDmIn"
    # bypasses the check because none of these equal the lowercase literal "admin".
    # An attacker with knowledge of the system username format can trivially bypass
    # this guard.
    local username="$1"
    [ "$username" == "admin" ] || { echo "Access denied" >&2; return 1; }  # vuln-code-snippet vuln-line auth_case_normalization_bypass
    echo "Admin access granted"
}
# vuln-code-snippet end auth_case_normalization_bypass

# vuln-code-snippet start auth_prefix_match_only
validate_api_token() {
    # The glob pattern ${VALID_TOKEN}* matches any string that starts with the valid
    # token. An attacker who learns the first few characters of the token (e.g., from
    # a partial leak) can bypass validation by appending arbitrary characters to the
    # known prefix.
    local token="$1"
    if [[ "$token" == "${VALID_TOKEN}"* ]]; then  # vuln-code-snippet vuln-line auth_prefix_match_only
        echo "Token accepted"
    else
        echo "Token rejected" >&2; return 1
    fi
}
# vuln-code-snippet end auth_prefix_match_only

# vuln-code-snippet start auth_grep_metachar_bypass
check_user_allowed() {
    # grep treats the first argument as a regex pattern. Supplying .* matches any line
    # in the file, bypassing the allowlist entirely. Supplying ^.* or a partial match
    # of another user's name grants unintended access. User input should not be used
    # as a grep pattern for access control.
    local username="$1"
    if grep -q "$username" /etc/app/allowed_users.txt; then  # vuln-code-snippet vuln-line auth_grep_metachar_bypass
        grant_access
    else
        echo "User not allowed" >&2; return 1
    fi
}
# vuln-code-snippet end auth_grep_metachar_bypass

# vuln-code-snippet start auth_md5_password_hash
verify_password() {
    # MD5 is cryptographically broken for password hashing. MD5 collisions are
    # computable, rainbow tables cover most common passwords, and GPU cracking
    # achieves billions of hashes per second. Password authentication requires a
    # memory-hard KDF like bcrypt, scrypt, or Argon2.
    local provided_password="$1"
    local stored_md5="$2"
    local computed
    computed=$(echo -n "$provided_password" | md5sum | awk '{print $1}')
    [ "$computed" = "$stored_md5" ] && echo "Authenticated"  # vuln-code-snippet vuln-line auth_md5_password_hash
}
# vuln-code-snippet end auth_md5_password_hash

# vuln-code-snippet start auth_admin_mode_skip
handle_request() {
    # When ADMIN_MODE is not explicitly set to "true", authentication is skipped
    # entirely. This inverts the security model: the safe path (authenticated) requires
    # an explicit flag, while the default path (unauthenticated) is normal execution.
    # Any invocation without ADMIN_MODE=true bypasses authentication.
    local action="$1"
    if [ "$ADMIN_MODE" = "true" ]; then
        authenticate_user
    else
        echo "Non-admin mode, skipping auth"
    fi
    execute_action "$action"  # vuln-code-snippet vuln-line auth_admin_mode_skip
}
# vuln-code-snippet end auth_admin_mode_skip

# vuln-code-snippet start auth_session_world_tmp
read_session_data() {
    # Session data is stored in /tmp with a predictable, user-controlled filename.
    # Any process on the system can read, create, or overwrite /tmp/session_* files.
    # An attacker creates /tmp/session_admin to impersonate the admin user.
    local session_user="$1"
    local session_file="/tmp/session_${session_user}"
    if [ -f "$session_file" ]; then
        cat "$session_file"  # vuln-code-snippet vuln-line auth_session_world_tmp
    fi
}
# vuln-code-snippet end auth_session_world_tmp

# vuln-code-snippet start auth_localhost_skip
process_api_request() {
    # Bypassing authentication for localhost requests assumes only trusted processes
    # can connect from 127.0.0.1. Combined with SSRF, any web-accessible endpoint that
    # makes server-side requests becomes a path to unauthenticated access. remote_addr
    # from HTTP headers is also forgeable at multiple layers.
    local remote_addr="$1"
    local action="$2"
    if [[ "$remote_addr" == "127.0.0.1" ]]; then
        echo "Localhost request, bypassing auth"
        execute_privileged_action "$action"  # vuln-code-snippet vuln-line auth_localhost_skip
        return 0
    fi
    authenticate_and_execute "$action"
}
# vuln-code-snippet end auth_localhost_skip

# vuln-code-snippet start auth_empty_bearer_pass
check_authorization() {
    # The check only verifies that the header is non-empty, not that it contains a
    # valid credential. Any non-empty string — including "invalid", "null", or a random
    # value — passes this check. The authorization header value is never validated.
    local auth_header="$1"
    if [[ -n "$auth_header" ]]; then  # vuln-code-snippet vuln-line auth_empty_bearer_pass
        echo "Authorization header present, access granted"
    else
        echo "No authorization header" >&2; return 1
    fi
}
# vuln-code-snippet end auth_empty_bearer_pass

# vuln-code-snippet start auth_hardcoded_comparison
admin_login() {
    # The expected password is hardcoded in the script. Anyone with read access to the
    # script file (source code, deployed binary, memory) learns the password. Hardcoded
    # credentials cannot be rotated and are exposed in version control history.
    local pass
    read -r -p "Enter admin password: " pass
    if [ "$pass" = "admin123" ]; then  # vuln-code-snippet vuln-line auth_hardcoded_comparison
        echo "Login successful"
    else
        echo "Incorrect password" >&2; return 1
    fi
}
# vuln-code-snippet end auth_hardcoded_comparison

# vuln-code-snippet start auth_jwt_none_algo
verify_jwt_token() {
    # The JWT "none" algorithm bypass: if alg is explicitly "none", signature
    # verification is skipped. An attacker forges a JWT with {"alg":"none"} in the
    # header and any claims in the payload — no signature required. The fix is to
    # reject "none" and enforce an explicit allowlist of accepted algorithms.
    local jwt="$1"
    local header_b64 header algo
    header_b64=$(echo "$jwt" | cut -d. -f1)
    header=$(echo "$header_b64" | base64 -d 2>/dev/null)
    algo=$(echo "$header" | python3 -c "import sys,json; print(json.load(sys.stdin).get('alg',''))" 2>/dev/null)
    if [ "$algo" != "none" ]; then  # vuln-code-snippet vuln-line auth_jwt_none_algo
        verify_signature "$jwt"
    fi
}
# vuln-code-snippet end auth_jwt_none_algo

# vuln-code-snippet start auth_non_prod_skip
require_authentication() {
    # Authentication is skipped in all non-production environments. If APP_ENV is unset
    # (the default in many deployments), authentication is bypassed. Attackers who can
    # set APP_ENV or deploy to any non-prod environment gain unauthenticated access.
    local action="$1"
    if [ "$APP_ENV" != "production" ]; then
        echo "Non-production environment, skipping auth"
        "$action"  # vuln-code-snippet vuln-line auth_non_prod_skip
        return 0
    fi
    run_with_auth "$action"
}
# vuln-code-snippet end auth_non_prod_skip

# vuln-code-snippet start auth_passwd_lookup
authenticate_system_user() {
    # /etc/passwd contains no password hashes on modern systems (they are in
    # /etc/shadow). Checking /etc/passwd for a username only verifies that the user
    # account exists — not that the provided password is correct. Any valid system
    # username bypasses this authentication.
    local username="$1"
    local provided_password="$2"
    local passwd_line
    passwd_line=$(grep "^${username}:" /etc/passwd)
    if [ -n "$passwd_line" ]; then  # vuln-code-snippet vuln-line auth_passwd_lookup
        echo "User exists, access granted"
    fi
}
# vuln-code-snippet end auth_passwd_lookup

# vuln-code-snippet start auth_http_response_substring
validate_token_remote() {
    # The substring match *"valid"* can be triggered by error messages containing the
    # word "valid". For example, "Invalid token format" or "Validation error occurred"
    # both contain "valid" and would grant access. Use exact JSON field parsing instead
    # of substring matching on HTTP responses.
    local token="$1"
    local result
    result=$(curl -s "http://auth-service.internal/validate?token=${token}")
    if [[ "$result" == *"valid"* ]]; then  # vuln-code-snippet vuln-line auth_http_response_substring
        echo "Token validated"
    else
        echo "Validation failed" >&2; return 1
    fi
}
# vuln-code-snippet end auth_http_response_substring

# vuln-code-snippet start auth_url_token_cleartext
call_api_with_token() {
    # The token is transmitted over HTTP (unencrypted) as a URL query parameter.
    # URL parameters appear in server access logs, proxy logs, browser history, and
    # HTTP Referer headers. Both the cleartext transmission (CWE-319) and the URL
    # parameter placement expose the credential.
    local token="$1"
    curl -s "http://api.example.com/data?token=${token}"  # vuln-code-snippet vuln-line auth_url_token_cleartext
}
# vuln-code-snippet end auth_url_token_cleartext

# vuln-code-snippet start auth_partial_regex_match
check_admin_role() {
    # The regex matches any string containing the substring "admin" — not just the
    # exact string. Values like "notadmin", "superadmin_blocked", or "badmin" all
    # match. Use an anchored exact match ([[ "$input_role" == "admin" ]]) or anchored
    # regex ([[ "$input_role" =~ ^admin$ ]]).
    local input_role="$1"
    if [[ "$input_role" =~ "admin" ]]; then  # vuln-code-snippet vuln-line auth_partial_regex_match
        echo "Admin role confirmed"
    else
        echo "Not an admin" >&2; return 1
    fi
}
# vuln-code-snippet end auth_partial_regex_match

# --- Safe variants ---

# vuln-code-snippet start auth_format_then_hmac
authenticate_api_request() {
    # Format validation rejects malformed input before HMAC computation. The
    # HMAC-SHA256 comparison verifies the token was issued by a party holding the
    # secret. This is a real authentication check, not just a non-empty check.
    local token="$1"
    if [[ ! "$token" =~ ^[A-Za-z0-9]{64}$ ]]; then
        echo "Invalid token format" >&2; return 1
    fi
    local expected
    expected=$(echo -n "$token" | openssl dgst -sha256 -hmac "${HMAC_SECRET}" -hex | awk '{print $2}')
    [ "$expected" = "${TOKEN_MAC_DB[$token]}" ]  # vuln-code-snippet safe-line auth_format_then_hmac
}
# vuln-code-snippet end auth_format_then_hmac

# vuln-code-snippet start auth_hmac_sha256_compute
verify_webhook_signature() {
    # HMAC-SHA256 requires knowledge of WEBHOOK_SECRET to produce a valid signature.
    # An attacker cannot forge a valid signature without the secret.
    local payload="$1"
    local signature="$2"
    local computed
    computed=$(echo -n "$payload" | openssl dgst -sha256 -hmac "$WEBHOOK_SECRET" -hex | awk '{print $2}')
    [ "$computed" = "$signature" ]  # vuln-code-snippet safe-line auth_hmac_sha256_compute
}
# vuln-code-snippet end auth_hmac_sha256_compute

# vuln-code-snippet start auth_uid_check
require_root() {
    # id -u reads the actual effective UID from the kernel, not from environment
    # variables. This cannot be bypassed by setting USER=root or HOME=/root — only
    # genuine root privileges satisfy the check.
    if [[ "$(id -u)" -ne 0 ]]; then  # vuln-code-snippet safe-line auth_uid_check
        echo "This command requires root" >&2
        exit 1
    fi
}
# vuln-code-snippet end auth_uid_check

# vuln-code-snippet start auth_file_ownership_check
verify_keyfile_integrity() {
    # Checking both ownership (root) and permissions (400 = read-only by owner) ensures
    # the key was installed by a privileged process and has not been tampered with by
    # unprivileged users.
    local keyfile="$1"
    local owner perms
    owner=$(stat -c '%U' "$keyfile")
    perms=$(stat -c '%a' "$keyfile")
    if [[ "$owner" != "root" || "$perms" != "400" ]]; then  # vuln-code-snippet safe-line auth_file_ownership_check
        echo "Key file ownership or permissions incorrect" >&2
        return 1
    fi
}
# vuln-code-snippet end auth_file_ownership_check

# vuln-code-snippet start auth_totp_code_verify
check_totp_code() {
    # oathtool --totp generates the current TOTP code from the shared secret. The
    # comparison requires knowledge of the secret. Format validation prevents
    # non-numeric input from reaching the comparison.
    local provided_otp="$1"
    if [[ ! "$provided_otp" =~ ^[0-9]{6}$ ]]; then
        echo "Invalid OTP format" >&2; return 1
    fi
    local expected
    expected=$(oathtool --totp "$TOTP_SECRET")
    [ "$expected" = "$provided_otp" ]  # vuln-code-snippet safe-line auth_totp_code_verify
}
# vuln-code-snippet end auth_totp_code_verify

# vuln-code-snippet start auth_gpg_verify
verify_signed_payload() {
    # GPG signature verification requires that the signing key exists in the trusted
    # keyring. An attacker cannot forge a signature without the private key
    # corresponding to a trusted public key.
    local payload_file="$1"
    local sig_file="$2"
    gpg --verify "$sig_file" "$payload_file"  # vuln-code-snippet safe-line auth_gpg_verify
}
# vuln-code-snippet end auth_gpg_verify

# vuln-code-snippet start auth_strict_role_allowlist
authorize_action() {
    # Only three known roles can reach the action execution. All other role values are
    # rejected. The case statement provides an explicit allowlist rather than implicit
    # acceptance.
    local user_role="$1"
    local action="$2"
    case "$user_role" in
        admin|operator|viewer)
            perform_action "$user_role" "$action"  # vuln-code-snippet safe-line auth_strict_role_allowlist
            ;;
        *)
            echo "Unknown role: $user_role" >&2
            return 1
            ;;
    esac
}
# vuln-code-snippet end auth_strict_role_allowlist

# vuln-code-snippet start auth_session_600_check
load_session() {
    # Verifying that the session file has 600 permissions (owner read-write only)
    # ensures the file was created by the application and has not been tampered with
    # by another user process. Sessions stored with world-readable permissions could
    # be forged.
    local session_id="$1"
    local session_file="/var/app/sessions/${session_id}"
    local perms
    perms=$(stat -c '%a' "$session_file" 2>/dev/null)
    if [[ "$perms" != "600" ]]; then
        echo "Session file has wrong permissions, rejecting" >&2; return 1
    fi
    cat "$session_file"  # vuln-code-snippet safe-line auth_session_600_check
}
# vuln-code-snippet end auth_session_600_check

# vuln-code-snippet start auth_logname_check
verify_expected_user() {
    # logname reads the login name from the system's utmp/wtmp database, not from
    # environment variables. Unlike $USER or $LOGNAME, it cannot be overridden by
    # setting environment variables in the current shell session.
    local expected_user="$1"
    local actual
    actual=$(logname)
    [ "$actual" = "$expected_user" ]  # vuln-code-snippet safe-line auth_logname_check
}
# vuln-code-snippet end auth_logname_check

# vuln-code-snippet start auth_x509_verify
verify_client_certificate() {
    # Certificate verification checks the cert against a trusted CA chain and validates
    # its intended purpose (sslclient). An attacker cannot forge a certificate signed by
    # a trusted CA without access to the CA's private key.
    local cert_file="$1"
    openssl verify -CAfile /etc/ssl/certs/ca-certificates.crt \
        -purpose sslclient "$cert_file"  # vuln-code-snippet safe-line auth_x509_verify
}
# vuln-code-snippet end auth_x509_verify

# vuln-code-snippet start auth_cert_cn_check
check_client_identity() {
    # The Common Name is extracted from a verified certificate. Combined with prior
    # certificate verification against a trusted CA, this checks that the certificate
    # belongs to the expected entity.
    local cert_file="$1"
    local expected_cn="$2"
    local cn
    cn=$(openssl x509 -noout -subject -in "$cert_file" | sed 's/.*CN = //')
    [ "$cn" = "$expected_cn" ]  # vuln-code-snippet safe-line auth_cert_cn_check
}
# vuln-code-snippet end auth_cert_cn_check

# vuln-code-snippet start auth_bcrypt_compare
check_stored_password() {
    # bcrypt.checkpw performs a timing-safe comparison with the stored hash. bcrypt
    # includes a cost factor that makes brute-force attacks computationally expensive
    # and resists rainbow table attacks via per-hash salts.
    local provided="$1"
    local stored_hash="$2"
    python3 -c "
import bcrypt, sys
provided = sys.argv[1].encode()
stored = sys.argv[2].encode()
result = bcrypt.checkpw(provided, stored)
sys.exit(0 if result else 1)
" "$provided" "$stored_hash"  # vuln-code-snippet safe-line auth_bcrypt_compare
}
# vuln-code-snippet end auth_bcrypt_compare

# vuln-code-snippet start auth_rate_limit_check
authenticate_with_rate_limit() {
    # Rate limiting prevents brute-force attacks by counting failed attempts per user.
    # The actual token verification (verify_hmac_token) only executes if the rate limit
    # is not exceeded.
    local username="$1"
    local token="$2"
    local attempts
    attempts=$(cat "/var/app/rate_limit/${username}" 2>/dev/null || echo 0)
    if [[ "$attempts" -ge 5 ]]; then
        echo "Rate limit exceeded" >&2; return 1
    fi
    verify_hmac_token "$username" "$token"  # vuln-code-snippet safe-line auth_rate_limit_check
}
# vuln-code-snippet end auth_rate_limit_check

# vuln-code-snippet start auth_mandatory_header_check
api_require_auth() {
    # The function rejects requests with no API key header before passing to
    # validation. The mandatory check (rejects empty) is then followed by actual key
    # validation, not just presence checking.
    local api_key_header="$1"
    if [[ -z "$api_key_header" ]]; then
        echo "401 Unauthorized: API key required" >&2
        return 1
    fi
    validate_api_key "$api_key_header"  # vuln-code-snippet safe-line auth_mandatory_header_check
}
# vuln-code-snippet end auth_mandatory_header_check

# vuln-code-snippet start auth_pam_authenticate
system_login_check() {
    # pamtester invokes the PAM (Pluggable Authentication Modules) stack for the
    # specified service. PAM authentication uses the system's configured authentication
    # backends (shadow passwords, Kerberos, LDAP) and cannot be bypassed by string
    # comparison manipulation.
    local username="$1"
    local password="$2"
    pamtester login "$username" authenticate <<< "$password"  # vuln-code-snippet safe-line auth_pam_authenticate
}
# vuln-code-snippet end auth_pam_authenticate
