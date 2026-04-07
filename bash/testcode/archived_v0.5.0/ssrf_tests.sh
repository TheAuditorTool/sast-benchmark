#!/bin/bash
# SSRF Test Cases (CWE-918)
# HTTP clients with user-controlled vs hardcoded URLs

# vuln-code-snippet start ssrf_curl_user_url
fetch_remote_resource() {
    local url="$1"
    curl -sf "$url"  # vuln-code-snippet vuln-line ssrf_curl_user_url
}
# vuln-code-snippet end ssrf_curl_user_url

# vuln-code-snippet start ssrf_hardcoded_url
check_api_health() {
    local endpoint="https://api.internal.example.com/health"
    curl -sf "$endpoint"  # vuln-code-snippet safe-line ssrf_hardcoded_url
}
# vuln-code-snippet end ssrf_hardcoded_url

# vuln-code-snippet start ssrf_url_path_injection
get_api_resource() {
    local resource_path="$1"
    local base_url="https://api.example.com"
    curl -sf "${base_url}/${resource_path}"  # vuln-code-snippet vuln-line ssrf_url_path_injection
}
# vuln-code-snippet end ssrf_url_path_injection

# vuln-code-snippet start ssrf_allowlist_validated
fetch_validated_url() {
    local url="$1"
    if [[ ! "$url" =~ ^https://(api|cdn)\.example\.com/ ]]; then
        echo "URL not in allowlist" >&2
        return 1
    fi
    curl -sf "$url"  # vuln-code-snippet safe-line ssrf_allowlist_validated
}
# vuln-code-snippet end ssrf_allowlist_validated

# vuln-code-snippet start ssrf_wget_redirect_follow
download_remote_file() {
    local url="$1"
    local dest="$2"
    # wget follows redirects by default — user URL could redirect to internal network
    wget -q -O "$dest" "$url"  # vuln-code-snippet vuln-line ssrf_wget_redirect_follow
}
# vuln-code-snippet end ssrf_wget_redirect_follow

# --- Phase 2 TN additions (OWASP 50/50 rebalancing, 2026-03-22) ---

# vuln-code-snippet start ssrf_hardcoded_api
get_github_commits() {
    #org and repo are hardcoded constants defined in the function.
    # The URL is assembled entirely from constants — no user control over host.
    local GITHUB_ORG="mycompany"
    local GITHUB_REPO="webapp"
    curl -sf "https://api.github.com/repos/${GITHUB_ORG}/${GITHUB_REPO}/commits"  # vuln-code-snippet safe-line ssrf_hardcoded_api
}
# vuln-code-snippet end ssrf_hardcoded_api

# vuln-code-snippet start ssrf_base_url_constant
notify_monitoring() {
    #base URL is a hardcoded constant. User-provided message goes only
    # in the POST body, not in the URL. Attacker cannot control the destination.
    local message="$1"
    local BASE_URL="https://monitoring.corp.internal"
    curl -sf -X POST "${BASE_URL}/api/v1/alert" \
        -H "Content-Type: application/json" \
        -d "{\"message\": \"${message}\"}"  # vuln-code-snippet safe-line ssrf_base_url_constant
}
# vuln-code-snippet end ssrf_base_url_constant

# vuln-code-snippet start ssrf_git_internal
clone_approved_repo() {
    #git host is hardcoded (git.corp.internal). Repo name is validated
    # against a strict regex — only lowercase alphanumeric + hyphens allowed.
    local name="$1"
    if [[ ! "$name" =~ ^[a-z][a-z0-9-]+$ ]]; then
        echo "Invalid repo name: $name" >&2
        return 1
    fi
    git clone "git@git.corp.internal:ops/${name}.git" "/tmp/${name}"  # vuln-code-snippet safe-line ssrf_git_internal
}
# vuln-code-snippet end ssrf_git_internal

# vuln-code-snippet start ssrf_curl_jq_escaped
slack_notify() {
    #Slack URL is a hardcoded constant. Message body is safely escaped
    # via jq --arg which handles JSON special characters. No user control of URL.
    local message="$1"
    local SLACK_URL="https://hooks.slack.com/services/T000/B000/XXXXX"
    local json
    json=$(jq -n --arg msg "$message" '{"text": $msg}')
    curl -sf -X POST -H "Content-Type: application/json" \
        -d "$json" "$SLACK_URL"  # vuln-code-snippet safe-line ssrf_curl_jq_escaped
}
# vuln-code-snippet end ssrf_curl_jq_escaped
