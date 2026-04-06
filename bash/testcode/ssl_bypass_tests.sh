#!/bin/bash
# SSL/TLS Bypass Test Cases (CWE-295)
# Certificate verification disabled vs proper verification

# vuln-code-snippet start ssl_curl_insecure_flag
fetch_internal_api() {
    local url="$1"
    curl --insecure -sf "$url"  # vuln-code-snippet vuln-line ssl_curl_insecure_flag
}
# vuln-code-snippet end ssl_curl_insecure_flag

# vuln-code-snippet start ssl_curl_default
fetch_external_api() {
    local url="$1"
    curl -sf "$url"  # vuln-code-snippet safe-line ssl_curl_default
}
# vuln-code-snippet end ssl_curl_default

# vuln-code-snippet start ssl_git_no_verify
clone_internal_repo() {
    local repo_url="$1"
    local dest="$2"
    GIT_SSL_NO_VERIFY=1 git clone "$repo_url" "$dest"  # vuln-code-snippet vuln-line ssl_git_no_verify
}
# vuln-code-snippet end ssl_git_no_verify

# vuln-code-snippet start ssl_curl_cacert
fetch_with_custom_ca() {
    local url="$1"
    curl --cacert /etc/ssl/internal-ca.pem -sf "$url"  # vuln-code-snippet safe-line ssl_curl_cacert
}
# vuln-code-snippet end ssl_curl_cacert

# vuln-code-snippet start ssl_ssh_accept_new
ssh_first_connect() {
    local host="$1"
    local cmd="$2"
    # accept-new: accepts first-time keys but rejects changed keys (TOFU model)
    ssh -o StrictHostKeyChecking=accept-new "$host" "$cmd"  # vuln-code-snippet safe-line ssl_ssh_accept_new
}
# vuln-code-snippet end ssl_ssh_accept_new

# --- Tier 2 additions (Phase 3, verified 2026-03-19) ---

# vuln-code-snippet start ssl_node_tls_reject_disabled
start_node_noverify() {
    local app_dir="$1"
    # Disables TLS certificate verification for ALL https requests in Node.js.
    # Common in CI/CD scripts to bypass self-signed cert issues. CWE-295.
    NODE_TLS_REJECT_UNAUTHORIZED=0 node "${app_dir}/server.js"  # vuln-code-snippet vuln-line ssl_node_tls_reject_disabled
}
# vuln-code-snippet end ssl_node_tls_reject_disabled

# vuln-code-snippet start ssl_node_tls_default
start_node_secure() {
    local app_dir="$1"
    # Default Node.js behavior verifies TLS certificates.
    # No override of NODE_TLS_REJECT_UNAUTHORIZED — verification enabled.
    node "${app_dir}/server.js"  # vuln-code-snippet safe-line ssl_node_tls_default
}
# vuln-code-snippet end ssl_node_tls_default

# vuln-code-snippet start ssl_git_ssl_no_verify_env
clone_with_env_bypass() {
    local repo_url="$1"
    local dest="$2"
    # GIT_SSL_NO_VERIFY=true exported for all subsequent git operations —
    # disables certificate verification globally for the session.
    export GIT_SSL_NO_VERIFY=true
    git clone "$repo_url" "$dest"  # vuln-code-snippet vuln-line ssl_git_ssl_no_verify_env
}
# vuln-code-snippet end ssl_git_ssl_no_verify_env

# vuln-code-snippet start ssl_npm_strict_ssl_false
install_npm_packages() {
    local project_dir="$1"
    # npm strict-ssl=false disables TLS verification for ALL npm registry
    # requests — MITM can serve malicious packages.
    cd "$project_dir" && npm install --strict-ssl=false  # vuln-code-snippet vuln-line ssl_npm_strict_ssl_false
}
# vuln-code-snippet end ssl_npm_strict_ssl_false

# vuln-code-snippet start ssl_pip_trusted_host
install_pip_no_verify() {
    local package="$1"
    # --trusted-host disables TLS verification for the specified host.
    # MITM can serve tampered packages from that host.
    pip install --trusted-host pypi.internal "$package"  # vuln-code-snippet vuln-line ssl_pip_trusted_host
}
# vuln-code-snippet end ssl_pip_trusted_host

# vuln-code-snippet start ssl_pythonhttpsverify_disabled
run_python_no_verify() {
    local script="$1"
    # PYTHONHTTPSVERIFY=0 disables certificate verification for ALL
    # urllib and requests calls in the Python process.
    PYTHONHTTPSVERIFY=0 python3 "$script"  # vuln-code-snippet vuln-line ssl_pythonhttpsverify_disabled
}
# vuln-code-snippet end ssl_pythonhttpsverify_disabled

# vuln-code-snippet start ssl_git_sslcainfo
clone_with_custom_ca() {
    local repo_url="$1"
    local dest="$2"
    # git http.sslCAInfo points to a custom CA bundle — verification
    # is still enabled, just uses a different trust anchor.
    git -c http.sslCAInfo=/etc/ssl/internal-ca.pem clone "$repo_url" "$dest"  # vuln-code-snippet safe-line ssl_git_sslcainfo
}
# vuln-code-snippet end ssl_git_sslcainfo

# vuln-code-snippet start ssl_pip_cert
install_pip_custom_cert() {
    local package="$1"
    # --cert points to a PEM bundle for verification — TLS is still active
    # and certificates are validated against the custom CA.
    pip install --cert /etc/ssl/internal-ca.pem "$package"  # vuln-code-snippet safe-line ssl_pip_cert
}
# vuln-code-snippet end ssl_pip_cert

# vuln-code-snippet start ssl_npm_cafile
install_npm_custom_ca() {
    local project_dir="$1"
    # npm cafile sets a custom CA — TLS verification remains enabled.
    cd "$project_dir" && npm install --cafile=/etc/ssl/internal-ca.pem  # vuln-code-snippet safe-line ssl_npm_cafile
}
# vuln-code-snippet end ssl_npm_cafile
