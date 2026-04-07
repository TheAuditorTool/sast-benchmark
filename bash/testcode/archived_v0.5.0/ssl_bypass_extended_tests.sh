#!/bin/bash
# SSL Verification Bypass Test Cases (CWE-295)
# Certificate validation bypass, host key checking, TLS verification

# vuln-code-snippet start ssl_git_no_verify_inline
clone_repo_for_deploy() {
    local repo="$1"
    local dest="$2"
    # GIT_SSL_NO_VERIFY as an inline env prefix applies only to this invocation
    # but fully disables SSL certificate verification — MITM can serve any cert.
    GIT_SSL_NO_VERIFY=true git clone "$repo" "$dest"  # vuln-code-snippet vuln-line ssl_git_no_verify_inline
}
# vuln-code-snippet end ssl_git_no_verify_inline

# vuln-code-snippet start ssl_cacert_devnull
fetch_internal_metrics() {
    local endpoint="$1"
    local output_file="$2"
    # /dev/null as a CA cert file yields an empty CA store.
    # curl treats zero-byte CA file as "no trusted CAs" — all certs accepted.
    curl --cacert /dev/null -o "$output_file" "$endpoint"  # vuln-code-snippet vuln-line ssl_cacert_devnull
}
# vuln-code-snippet end ssl_cacert_devnull

# vuln-code-snippet start ssl_openssl_verify_zero
probe_tls_endpoint() {
    local host="$1"
    local port="${2:-443}"
    # -verify 0 sets the verification depth to zero, which disables chain
    # validation entirely — the certificate is never checked against a CA.
    openssl s_client -connect "${host}:${port}" -verify 0 </dev/null  # vuln-code-snippet vuln-line ssl_openssl_verify_zero
}
# vuln-code-snippet end ssl_openssl_verify_zero

# vuln-code-snippet start ssl_wget_no_hsts
download_release_artifact() {
    local url="$1"
    local dest="$2"
    # --no-hsts disables HSTS enforcement, allowing an attacker to downgrade
    # HTTPS to HTTP on sites that would otherwise mandate TLS.
    wget --no-hsts -O "$dest" "$url"  # vuln-code-snippet vuln-line ssl_wget_no_hsts
}
# vuln-code-snippet end ssl_wget_no_hsts

# vuln-code-snippet start ssl_ssh_config_injection
configure_deployment_ssh() {
    # Writing "Host *" with StrictHostKeyChecking no to ~/.ssh/config affects
    # every subsequent SSH connection from this user — not just the deployment
    # target. A rogue host presenting any key will be silently accepted.
    echo -e "Host *\n  StrictHostKeyChecking no" >> ~/.ssh/config  # vuln-code-snippet vuln-line ssl_ssh_config_injection
}
# vuln-code-snippet end ssl_ssh_config_injection

# vuln-code-snippet start ssl_ssh_indirect_bypass
run_remote_build() {
    local host="$1"
    local build_cmd="$2"
    SSH_OPTS="-o StrictHostKeyChecking=no"
    # shellcheck disable=SC2086
    ssh $SSH_OPTS "$host" "$build_cmd"  # vuln-code-snippet vuln-line ssl_ssh_indirect_bypass
}
# vuln-code-snippet end ssl_ssh_indirect_bypass

# vuln-code-snippet start ssl_jvm_ssl_disabled
start_app_server() {
    local jar_path="$1"
    # com.sun.net.ssl.checkRevocation=false tells the JVM not to check CRL/OCSP.
    # A revoked certificate (e.g. from a compromised CA) is accepted as valid.
    JAVA_OPTS="-Dcom.sun.net.ssl.checkRevocation=false" java -jar "$jar_path"  # vuln-code-snippet vuln-line ssl_jvm_ssl_disabled
}
# vuln-code-snippet end ssl_jvm_ssl_disabled

# vuln-code-snippet start ssl_node_tls_inline
run_integration_tests() {
    local entry="$1"
    # Inline env prefix sets NODE_TLS_REJECT_UNAUTHORIZED=0 for the child
    # process only, but that is sufficient for the node process to accept
    # any certificate including self-signed or expired ones.
    NODE_TLS_REJECT_UNAUTHORIZED=0 node "$entry"  # vuln-code-snippet vuln-line ssl_node_tls_inline
}
# vuln-code-snippet end ssl_node_tls_inline

# vuln-code-snippet start ssl_ansible_validate_certs
check_service_reachability() {
    local target="$1"
    # validate_certs=false in the Ansible uri module disables TLS certificate
    # verification — the module will accept any cert, including forged ones.
    ansible all -m uri -a "url=https://${target} validate_certs=false"  # vuln-code-snippet vuln-line ssl_ansible_validate_certs
}
# vuln-code-snippet end ssl_ansible_validate_certs

# vuln-code-snippet start ssl_curl_empty_pinnedpubkey
upload_telemetry() {
    local payload="$1"
    local endpoint="$2"
    # An empty string for --pinnedpubkey disables public key pinning entirely.
    # curl will not verify the server's public key against any expected value.
    curl --pinnedpubkey "" -d "$payload" "$endpoint"  # vuln-code-snippet vuln-line ssl_curl_empty_pinnedpubkey
}
# vuln-code-snippet end ssl_curl_empty_pinnedpubkey

# vuln-code-snippet start ssl_git_c_sslverify
pull_private_repo() {
    local repo="$1"
    local branch="${2:-main}"
    # -c http.sslVerify=false is a per-command config override that takes
    # precedence over global and system git config — SSL verification is off
    # for this clone regardless of what the system config says.
    git -c http.sslVerify=false clone --branch "$branch" "$repo"  # vuln-code-snippet vuln-line ssl_git_c_sslverify
}
# vuln-code-snippet end ssl_git_c_sslverify

# vuln-code-snippet start ssl_sshpass_no_check
deploy_to_legacy_host() {
    local host="$1"
    local user="$2"
    local pass="$3"
    local script="$4"
    # Double exposure: password is passed on the command line (visible in ps)
    # AND StrictHostKeyChecking=no means a rogue host can intercept the session.
    sshpass -p "$pass" ssh -o StrictHostKeyChecking=no "${user}@${host}" "$script"  # vuln-code-snippet vuln-line ssl_sshpass_no_check
}
# vuln-code-snippet end ssl_sshpass_no_check

# vuln-code-snippet start ssl_requests_verify_false
pull_metrics_snapshot() {
    local url="$1"
    local out="$2"
    # CURL_SSL_VERIFYPEER=0 is a libcurl environment variable that disables
    # peer certificate verification for the curl process — equivalent to -k.
    CURL_SSL_VERIFYPEER=0 curl -o "$out" "$url"  # vuln-code-snippet vuln-line ssl_requests_verify_false
}
# vuln-code-snippet end ssl_requests_verify_false

# vuln-code-snippet start ssl_pythonhttpsverify_inline
run_data_pipeline() {
    local script="$1"
    # PYTHONHTTPSVERIFY=0 disables certificate verification in Python's urllib
    # for the entire process lifetime — any HTTPS request in script.py skips
    # certificate validation.
    PYTHONHTTPSVERIFY=0 python3 "$script"  # vuln-code-snippet vuln-line ssl_pythonhttpsverify_inline
}
# vuln-code-snippet end ssl_pythonhttpsverify_inline

# vuln-code-snippet start ssl_conditional_debug_bypass
fetch_build_artifact() {
    local url="$1"
    local out="$2"
    local DEBUG="true"
    local CURL_FLAGS=""
    # DEBUG is hardcoded to "true" above — the branch is always taken and
    # CURL_FLAGS is always "-k". The conditional is nominal: this script
    # unconditionally disables TLS verification on every invocation.
    [[ "$DEBUG" == "true" ]] && CURL_FLAGS="-k"
    # shellcheck disable=SC2086
    curl $CURL_FLAGS -o "$out" "$url"  # vuln-code-snippet vuln-line ssl_conditional_debug_bypass
}
# vuln-code-snippet end ssl_conditional_debug_bypass

# vuln-code-snippet start ssl_explicit_cacert_bundle
download_release_package() {
    local url="$1"
    local out="$2"
    curl --cacert /etc/ssl/certs/ca-certificates.crt -o "$out" "$url"  # vuln-code-snippet safe-line ssl_explicit_cacert_bundle
}
# vuln-code-snippet end ssl_explicit_cacert_bundle

# vuln-code-snippet start ssl_ssh_strict_explicit
connect_to_bastion() {
    local user="$1"
    local host="$2"
    # StrictHostKeyChecking=yes rejects connections to hosts not in known_hosts,
    # which is stricter than the default "ask" behavior.
    ssh -o StrictHostKeyChecking=yes "${user}@${host}"  # vuln-code-snippet safe-line ssl_ssh_strict_explicit
}
# vuln-code-snippet end ssl_ssh_strict_explicit

# vuln-code-snippet start ssl_git_ca_pin
mirror_repository() {
    local repo="$1"
    local dest="$2"
    # GIT_SSL_CAINFO pins the CA bundle rather than disabling verification —
    # only certificates signed by the specified CA are accepted.
    GIT_SSL_CAINFO=/etc/ssl/certs/ca-bundle.crt git clone "$repo" "$dest"  # vuln-code-snippet safe-line ssl_git_ca_pin
}
# vuln-code-snippet end ssl_git_ca_pin

# vuln-code-snippet start ssl_openssl_verify_cert
validate_service_cert() {
    local cert="$1"
    local ca="$2"
    # openssl verify performs offline certificate chain validation only —
    # no connection is established, no bypass is possible.
    openssl verify -CAfile "$ca" "$cert"  # vuln-code-snippet safe-line ssl_openssl_verify_cert
}
# vuln-code-snippet end ssl_openssl_verify_cert

# vuln-code-snippet start ssl_requests_ca_bundle_env
run_report_generator() {
    local script="$1"
    # REQUESTS_CA_BUNDLE points the Python requests library at the system CA
    # store — verification remains enabled, just using an explicit bundle path.
    REQUESTS_CA_BUNDLE=/etc/ssl/certs/ca-certificates.crt python3 "$script"  # vuln-code-snippet safe-line ssl_requests_ca_bundle_env
}
# vuln-code-snippet end ssl_requests_ca_bundle_env

# vuln-code-snippet start ssl_curl_mutual_tls
call_payment_api() {
    local payload="$1"
    local endpoint="$2"
    # Mutual TLS: client presents its certificate to the server. No bypass flag
    # is present — server certificate validation is performed by default.
    curl --cert client.crt --key client.key -d "$payload" "$endpoint"  # vuln-code-snippet safe-line ssl_curl_mutual_tls
}
# vuln-code-snippet end ssl_curl_mutual_tls

# vuln-code-snippet start ssl_npm_cafile_set
configure_npm_registry() {
    local registry="$1"
    npm config set registry "$registry"
    # cafile configures npm to validate registry TLS using the system CA store.
    npm config set cafile /etc/ssl/certs/ca-certificates.crt  # vuln-code-snippet safe-line ssl_npm_cafile_set
}
# vuln-code-snippet end ssl_npm_cafile_set

# vuln-code-snippet start ssl_keyscan_pin
trust_deployment_host() {
    local host="$1"
    # ssh-keyscan implements TOFU (Trust On First Use): it adds the host key to
    # known_hosts so subsequent connections are verified against that pinned key.
    # This strengthens security compared to accepting unknown hosts interactively.
    ssh-keyscan -H "$host" >> ~/.ssh/known_hosts  # vuln-code-snippet safe-line ssl_keyscan_pin
}
# vuln-code-snippet end ssl_keyscan_pin

# vuln-code-snippet start ssl_wget_ca_cert
fetch_firmware_image() {
    local url="$1"
    local dest="$2"
    # --ca-certificate explicitly designates the CA bundle — wget will reject
    # any server certificate not signed by that bundle.
    wget --ca-certificate=/etc/ssl/certs/ca-certificates.crt -O "$dest" "$url"  # vuln-code-snippet safe-line ssl_wget_ca_cert
}
# vuln-code-snippet end ssl_wget_ca_cert

# vuln-code-snippet start ssl_curl_ca_bundle_env
init_curl_environment() {
    # Exporting CURL_CA_BUNDLE sets the CA bundle for all subsequent curl
    # invocations in this shell — verification remains fully enabled.
    export CURL_CA_BUNDLE=/etc/ssl/certs/ca-certificates.crt  # vuln-code-snippet safe-line ssl_curl_ca_bundle_env
}
# vuln-code-snippet end ssl_curl_ca_bundle_env

# vuln-code-snippet start ssl_curl_localhost_k
check_local_service_health() {
    local port="${1:-8443}"
    # -k is used exclusively against the loopback address (127.0.0.1/localhost).
    # There is no network path between the loopback and an external attacker,
    # so disabling certificate verification here provides no exploitable surface.
    curl -k "https://localhost:${port}/health"  # vuln-code-snippet safe-line ssl_curl_localhost_k
}
# vuln-code-snippet end ssl_curl_localhost_k

# vuln-code-snippet start ssl_python_cert_required
run_analytics_job() {
    local script="$1"
    # SSL_CERT_FILE points Python's ssl module at the system CA store.
    # Verification is performed — this is equivalent to specifying cafile= in code.
    SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt python3 "$script"  # vuln-code-snippet safe-line ssl_python_cert_required
}
# vuln-code-snippet end ssl_python_cert_required

# vuln-code-snippet start ssl_git_sslcainfo_set
bootstrap_git_config() {
    # Configures git globally to use the system CA store for TLS verification.
    # This replaces any previously missing or incorrect CA configuration and
    # ensures all subsequent git HTTPS operations validate certificates.
    git config --global http.sslCAInfo /etc/ssl/certs/ca-certificates.crt  # vuln-code-snippet safe-line ssl_git_sslcainfo_set
}
# vuln-code-snippet end ssl_git_sslcainfo_set

# vuln-code-snippet start ssl_ssh_known_hosts_strict
connect_to_production() {
    local user="$1"
    local host="$2"
    # StrictHostKeyChecking=yes + explicit system-wide known_hosts file combines
    # the strictest host key enforcement with a centrally managed trust store.
    ssh -o StrictHostKeyChecking=yes -o UserKnownHostsFile=/etc/ssh/known_hosts "${user}@${host}"  # vuln-code-snippet safe-line ssl_ssh_known_hosts_strict
}
# vuln-code-snippet end ssl_ssh_known_hosts_strict

# vuln-code-snippet start ssl_pip_cert_set
install_private_package() {
    local package="$1"
    # --cert passes a CA certificate file to pip — TLS verification is active
    # and uses this bundle rather than the system default.
    pip install --cert /etc/ssl/certs/ca-certificates.crt "$package"  # vuln-code-snippet safe-line ssl_pip_cert_set
}
# vuln-code-snippet end ssl_pip_cert_set
