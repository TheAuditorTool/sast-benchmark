#!/bin/bash
# SSL/TLS Bypass Test Cases (CWE-295)
# Certificate verification disabled vs proper verification

# vuln-code-snippet start ssl_curl_insecure_flag
fetch_internal_api() {
    local url="$1"
    curl --insecure -sf "$url"  # vuln-code-snippet vuln-line ssl_curl_insecure_flag
}
# vuln-code-snippet end ssl_curl_insecure_flag

# vuln-code-snippet start ssl_curl_default_safe
fetch_external_api() {
    local url="$1"
    curl -sf "$url"  # vuln-code-snippet safe-line ssl_curl_default_safe
}
# vuln-code-snippet end ssl_curl_default_safe

# vuln-code-snippet start ssl_git_no_verify
clone_internal_repo() {
    local repo_url="$1"
    local dest="$2"
    GIT_SSL_NO_VERIFY=1 git clone "$repo_url" "$dest"  # vuln-code-snippet vuln-line ssl_git_no_verify
}
# vuln-code-snippet end ssl_git_no_verify

# vuln-code-snippet start ssl_curl_cacert_safe
fetch_with_custom_ca() {
    local url="$1"
    curl --cacert /etc/ssl/internal-ca.pem -sf "$url"  # vuln-code-snippet safe-line ssl_curl_cacert_safe
}
# vuln-code-snippet end ssl_curl_cacert_safe

# vuln-code-snippet start ssl_ssh_accept_new_safe
ssh_first_connect() {
    local host="$1"
    local cmd="$2"
    # accept-new: accepts first-time keys but rejects changed keys (TOFU model)
    ssh -o StrictHostKeyChecking=accept-new "$host" "$cmd"  # vuln-code-snippet safe-line ssl_ssh_accept_new_safe
}
# vuln-code-snippet end ssl_ssh_accept_new_safe

# --- Tier 2 additions (Phase 3, verified 2026-03-19) ---

# vuln-code-snippet start ssl_node_tls_reject_disabled
start_node_insecure() {
    local app_dir="$1"
    # Disables TLS certificate verification for ALL https requests in Node.js.
    # Common in CI/CD scripts to bypass self-signed cert issues. CWE-295.
    NODE_TLS_REJECT_UNAUTHORIZED=0 node "${app_dir}/server.js"  # vuln-code-snippet vuln-line ssl_node_tls_reject_disabled
}
# vuln-code-snippet end ssl_node_tls_reject_disabled

# vuln-code-snippet start ssl_node_tls_default_safe
start_node_secure() {
    local app_dir="$1"
    # Default Node.js behavior verifies TLS certificates.
    # No override of NODE_TLS_REJECT_UNAUTHORIZED — verification enabled.
    node "${app_dir}/server.js"  # vuln-code-snippet safe-line ssl_node_tls_default_safe
}
# vuln-code-snippet end ssl_node_tls_default_safe
