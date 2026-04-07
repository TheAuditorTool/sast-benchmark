#!/bin/bash
# SSRF Extended Test Cases (CWE-918)
# Additional patterns beyond v0.4.0: wget-tar pipe, path injection, SSH tunnels,
# rsync/git/netcat SSRF, cloud storage SSRF, proxy SSRF, registry SSRF.

# vuln-code-snippet start ssrf_wget_tar_pipe
fetch_and_extract() {
    local user_url="$1"
    # wget fetches from user_url and pipes directly to tar for extraction. The attacker
    # can point user_url at an internal service (e.g., http://169.254.169.254/latest/meta-data/)
    # to exfiltrate cloud metadata, or deliver a malicious tar archive for path traversal
    # during extraction.
    wget -O- "$user_url" | tar xz  # vuln-code-snippet vuln-line ssrf_wget_tar_pipe
}
# vuln-code-snippet end ssrf_wget_tar_pipe

# vuln-code-snippet start ssrf_path_suffix_injection
call_internal_api() {
    local user_path="$1"
    # The scheme and host are hardcoded, but user_path is appended directly. Injecting
    # @evil.com/ produces http://internal-api.corp/@evil.com/ where @evil.com is parsed
    # as the actual host (the part before @ is treated as userinfo). Alternatively,
    # path traversal exposes unintended API endpoints.
    curl -s "http://internal-api.corp/${user_path}"  # vuln-code-snippet vuln-line ssrf_path_suffix_injection
}
# vuln-code-snippet end ssrf_path_suffix_injection

# vuln-code-snippet start ssrf_ssh_dynamic_tunnel
open_socks_proxy() {
    local host="$1"
    # ssh -D creates a SOCKS5 proxy tunnel to the specified host. When host is
    # user-controlled, the tunnel can be established to any reachable server,
    # routing subsequent traffic through that host and enabling SSRF.
    ssh -N -D 1080 "user@${host}"  # vuln-code-snippet vuln-line ssrf_ssh_dynamic_tunnel
}
# vuln-code-snippet end ssrf_ssh_dynamic_tunnel

# vuln-code-snippet start ssrf_rsync_user_host
pull_remote_data() {
    local remote_host="$1"
    # rsync connects to remote_host over SSH. When remote_host is attacker-controlled,
    # the rsync session connects to the attacker's server, potentially sending
    # authentication credentials and establishing an outbound connection from the
    # internal network.
    rsync -avz "user@${remote_host}:/data/" ./sync/  # vuln-code-snippet vuln-line ssrf_rsync_user_host
}
# vuln-code-snippet end ssrf_rsync_user_host

# vuln-code-snippet start ssrf_git_ls_remote
check_repo_refs() {
    local repo_url="$1"
    # git ls-remote queries the remote repository's refs over the network. With a
    # user-controlled URL, an attacker routes this request to any target (including
    # internal services), potentially triggering credential disclosure via git:// or
    # HTTP basic auth leakage.
    git ls-remote "$repo_url"  # vuln-code-snippet vuln-line ssrf_git_ls_remote
}
# vuln-code-snippet end ssrf_git_ls_remote

# vuln-code-snippet start ssrf_outbound_webhook
send_webhook_notification() {
    local webhook_url="$1"
    local payload="$2"
    # The webhook endpoint is fully user-controlled. An attacker supplies an internal
    # service URL (e.g., http://10.0.0.1/admin/endpoint), causing the server to make
    # authenticated requests to internal infrastructure.
    curl -s -X POST "$webhook_url" -H "Content-Type: application/json" -d "$payload"  # vuln-code-snippet vuln-line ssrf_outbound_webhook
}
# vuln-code-snippet end ssrf_outbound_webhook

# vuln-code-snippet start ssrf_netcat_exfil
send_config_to_host() {
    local host="$1"
    local port="$2"
    # /etc/app/config.yml is sent to user-controlled host:port over a plaintext TCP
    # connection. The attacker receives the configuration file (which may contain
    # credentials) and triggers an outbound connection from the internal network.
    nc "$host" "$port" < /etc/app/config.yml  # vuln-code-snippet vuln-line ssrf_netcat_exfil
}
# vuln-code-snippet end ssrf_netcat_exfil

# vuln-code-snippet start ssrf_scp_remote_host
retrieve_remote_config() {
    local remote_host="$1"
    # SCP connects to remote_host and authenticates with SSH credentials. An
    # attacker-controlled remote_host receives the SSH connection and may harvest
    # keys or credentials from the SCP handshake.
    scp "${remote_host}:config.tar.gz" ./  # vuln-code-snippet vuln-line ssrf_scp_remote_host
}
# vuln-code-snippet end ssrf_scp_remote_host

# vuln-code-snippet start ssrf_ftp_user_host
download_ftp_artifact() {
    local ftp_host="$1"
    # FTP connects to ftp_host in cleartext. User-controlled host enables SSRF to
    # any FTP-accessible internal service, plus credential exposure over the
    # unencrypted FTP protocol.
    ftp -n "$ftp_host" <<EOF  # vuln-code-snippet vuln-line ssrf_ftp_user_host
user anonymous anonymous@
get artifacts.tar.gz
quit
EOF
}
# vuln-code-snippet end ssrf_ftp_user_host

# vuln-code-snippet start ssrf_aws_user_bucket
sync_from_storage() {
    local user_bucket="$1"
    # The S3 bucket name is user-controlled. AWS CLI resolves bucket names through
    # DNS. A crafted bucket name with a custom endpoint (via --endpoint-url or aws
    # config) can route requests to internal services. Additionally, the aws CLI may
    # send SigV4 authentication headers to the attacker's endpoint.
    aws s3 cp "s3://${user_bucket}/deployment.tar.gz" ./  # vuln-code-snippet vuln-line ssrf_aws_user_bucket
}
# vuln-code-snippet end ssrf_aws_user_bucket

# vuln-code-snippet start ssrf_curl_user_proxy
fetch_with_proxy() {
    local proxy_url="$1"
    # The proxy is user-controlled. All request content (including headers and
    # authentication tokens) routes through the attacker's proxy, enabling credential
    # harvesting. Additionally, the proxy can inject responses to manipulate the
    # fetched content.
    curl --proxy "$proxy_url" "https://api.internal.corp/v1/status"  # vuln-code-snippet vuln-line ssrf_curl_user_proxy
}
# vuln-code-snippet end ssrf_curl_user_proxy

# vuln-code-snippet start ssrf_docker_user_registry
pull_application_image() {
    local image_ref="$1"
    # The image reference is user-controlled. Docker parses registry/image:tag —
    # supplying attacker.com/malicious:latest routes the pull request to attacker.com,
    # transmitting Docker authentication credentials and potentially pulling a
    # backdoored image.
    docker pull "$image_ref"  # vuln-code-snippet vuln-line ssrf_docker_user_registry
}
# vuln-code-snippet end ssrf_docker_user_registry

# vuln-code-snippet start ssrf_base_url_path_override
query_api_resource() {
    local user_path="$1"
    local BASE_URL="https://api.example.com"
    # Although BASE_URL is hardcoded, user_path is appended without validation.
    # Supplying @evil.com/ produces https://api.example.com@evil.com/ — in URL
    # parsing, the @ separates userinfo from hostname, so evil.com becomes the
    # actual destination. This is a classic URL injection/SSRF through host override.
    curl -s "${BASE_URL}${user_path}"  # vuln-code-snippet vuln-line ssrf_base_url_path_override
}
# vuln-code-snippet end ssrf_base_url_path_override

# vuln-code-snippet start ssrf_git_clone_exec
deploy_plugin_from_repo() {
    local repo_url="$1"
    # git clone fetches from user-controlled repo_url, then the install script is
    # executed without integrity verification. The attacker controls the content of
    # install.sh by controlling the repository, achieving both SSRF (outbound git
    # connection to arbitrary host) and RCE.
    git clone "$repo_url" /tmp/plugin_src  # vuln-code-snippet vuln-line ssrf_git_clone_exec
    bash /tmp/plugin_src/install.sh
}
# vuln-code-snippet end ssrf_git_clone_exec

# --- TNs (false positives / safe patterns) ---

# vuln-code-snippet start ssrf_github_api_hardcoded
get_latest_release() {
    # The URL is fully hardcoded. No user input is involved. Tools that flag all
    # outbound curl calls without taint analysis will FP here.
    curl -s "https://api.github.com/repos/example-org/app/releases/latest" \
        -H "Accept: application/vnd.github.v3+json"  # vuln-code-snippet safe-line ssrf_github_api_hardcoded
}
# vuln-code-snippet end ssrf_github_api_hardcoded

# vuln-code-snippet start ssrf_semver_download
download_release_artifact() {
    local version="$1"
    if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version" >&2
        return 1
    fi
    # The regex ^[0-9]+\.[0-9]+\.[0-9]+$ permits only digits and dots — no URL
    # special characters. The base URL is hardcoded; only the version component is
    # variable. No SSRF path exists.
    wget -q "https://releases.example.com/app/v${version}/app-linux-amd64.tar.gz"  # vuln-code-snippet safe-line ssrf_semver_download
}
# vuln-code-snippet end ssrf_semver_download

# vuln-code-snippet start ssrf_git_clone_hardcoded
clone_main_repo() {
    # Hardcoded repository URL. No user input involved.
    git clone "https://github.com/example-org/infrastructure.git" /opt/infra  # vuln-code-snippet safe-line ssrf_git_clone_hardcoded
}
# vuln-code-snippet end ssrf_git_clone_hardcoded

# vuln-code-snippet start ssrf_rsync_hardcoded_dest
backup_app_data() {
    # Both source and destination are hardcoded. No user input in the URL components.
    rsync -avz --delete /var/app/data/ "backup@storage.internal.corp:/backups/app/"  # vuln-code-snippet safe-line ssrf_rsync_hardcoded_dest
}
# vuln-code-snippet end ssrf_rsync_hardcoded_dest

# vuln-code-snippet start ssrf_token_only_variable
charge_customer() {
    local amount="$1"
    local customer_id="$2"
    # The API endpoint URL is hardcoded to https://api.stripe.com. Only the auth
    # token and POST body parameters are variable. The token and body data cannot
    # redirect the request to a different host.
    curl -s -H "Authorization: Bearer ${STRIPE_SECRET_KEY}" \
        "https://api.stripe.com/v1/charges" \
        -d "amount=${amount}&currency=usd&customer=${customer_id}"  # vuln-code-snippet safe-line ssrf_token_only_variable
}
# vuln-code-snippet end ssrf_token_only_variable

# vuln-code-snippet start ssrf_s3_hardcoded_bucket
upload_build_artifact() {
    local artifact_path="$1"
    # The S3 bucket is hardcoded. The user-controlled parameter is only the local
    # source path (artifact_path), not the destination bucket URL. No SSRF risk.
    aws s3 cp "$artifact_path" "s3://company-ci-artifacts/builds/"  # vuln-code-snippet safe-line ssrf_s3_hardcoded_bucket
}
# vuln-code-snippet end ssrf_s3_hardcoded_bucket

# vuln-code-snippet start ssrf_assoc_array_allowlist
call_internal_service() {
    local service_name="$1"
    declare -A SERVICE_URLS=(
        [auth]="https://auth.internal.corp/health"
        [metrics]="https://metrics.internal.corp/health"
        [cache]="https://cache.internal.corp/health"
    )
    # SERVICE_URLS is a hardcoded associative array. The user only provides a key
    # name. If service_name is not one of the declared keys, the expansion returns
    # empty string, causing a curl error rather than a connection to an unintended
    # host. This is an allowlist-by-design pattern.
    curl -s "${SERVICE_URLS[$service_name]}"  # vuln-code-snippet safe-line ssrf_assoc_array_allowlist
}
# vuln-code-snippet end ssrf_assoc_array_allowlist

# vuln-code-snippet start ssrf_pinned_tag_only
pull_versioned_image() {
    local version_tag="$1"
    # The registry hostname and image name are hardcoded. Only the version tag is
    # variable. A version tag cannot change the registry host — Docker parses the
    # full reference before resolving the registry.
    docker pull "registry.company.internal/app:${version_tag}"  # vuln-code-snippet safe-line ssrf_pinned_tag_only
}
# vuln-code-snippet end ssrf_pinned_tag_only

# vuln-code-snippet start ssrf_case_allowlist_url
health_check_service() {
    local target="$1"
    local url
    case "$target" in
        production)  url="https://app.prod.example.com/health" ;;
        staging)     url="https://app.staging.example.com/health" ;;
        canary)      url="https://app.canary.example.com/health" ;;
        *)           echo "Unknown target: $target" >&2; return 1 ;;
    esac
    # The URL is constructed from a hardcoded map of allowed targets. User input
    # only selects among three known URLs; it cannot introduce a new host.
    curl -s "$url"  # vuln-code-snippet safe-line ssrf_case_allowlist_url
}
# vuln-code-snippet end ssrf_case_allowlist_url

# vuln-code-snippet start ssrf_own_hostname_health
check_self_health() {
    # hostname -f returns the FQDN of the machine running the script. The resulting
    # URL points back to the machine itself. While the hostname is dynamic, it is
    # not user-controlled — it comes from the OS.
    curl -s "https://$(hostname -f)/health"  # vuln-code-snippet safe-line ssrf_own_hostname_health
}
# vuln-code-snippet end ssrf_own_hostname_health

# vuln-code-snippet start ssrf_semver_hashicorp
download_terraform() {
    local tf_version="$1"
    if [[ ! "$tf_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version format" >&2
        return 1
    fi
    # Semver validation allows only digits and dots. The base URL is hardcoded to
    # releases.hashicorp.com. No user-controlled URL components.
    wget -q "https://releases.hashicorp.com/terraform/${tf_version}/terraform_${tf_version}_linux_amd64.zip"  # vuln-code-snippet safe-line ssrf_semver_hashicorp
}
# vuln-code-snippet end ssrf_semver_hashicorp

# vuln-code-snippet start ssrf_branch_regex_validated
fetch_branch_changes() {
    local branch="$1"
    if [[ ! "$branch" =~ ^[a-zA-Z0-9/_.-]+$ ]]; then
        echo "Invalid branch name" >&2
        return 1
    fi
    # The branch name is validated to contain only alphanumerics, slashes,
    # underscores, dots, and hyphens. These characters cannot change the remote
    # host or inject git protocol commands.
    git fetch origin "$branch"  # vuln-code-snippet safe-line ssrf_branch_regex_validated
}
# vuln-code-snippet end ssrf_branch_regex_validated

# vuln-code-snippet start ssrf_path_alphanumeric
get_resource() {
    local resource_type="$1"
    local resource_id="$2"
    case "$resource_type" in
        users|orders|products) ;;
        *) echo "Invalid resource type" >&2; return 1 ;;
    esac
    if [[ ! "$resource_id" =~ ^[0-9]+$ ]]; then
        echo "Invalid resource ID" >&2
        return 1
    fi
    # resource_type is allowlisted to three known values. resource_id is validated
    # as numeric-only. Neither can contain URL special characters. The base URL is
    # hardcoded.
    curl -s "https://api.example.com/v2/${resource_type}/${resource_id}"  # vuln-code-snippet safe-line ssrf_path_alphanumeric
}
# vuln-code-snippet end ssrf_path_alphanumeric

# vuln-code-snippet start ssrf_internal_hardcoded
scrape_internal_metrics() {
    # Fully hardcoded URL with no user input. Internal Prometheus endpoint
    # scraping — no user input in any URL component.
    curl -s "https://prometheus.monitoring.internal.corp/api/v1/query?query=up"  # vuln-code-snippet safe-line ssrf_internal_hardcoded
}
# vuln-code-snippet end ssrf_internal_hardcoded
