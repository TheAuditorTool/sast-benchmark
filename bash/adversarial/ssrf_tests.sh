#!/bin/bash
# SSRF Test Cases (CWE-918)
# HTTP clients with user-controlled vs hardcoded URLs

# vuln-code-snippet start ssrf_curl_user_url
fetch_remote_resource() {
    local url="$1"
    curl -sf "$url"  # vuln-code-snippet vuln-line ssrf_curl_user_url
}
# vuln-code-snippet end ssrf_curl_user_url

# vuln-code-snippet start ssrf_hardcoded_url_safe
check_api_health() {
    local endpoint="https://api.internal.example.com/health"
    curl -sf "$endpoint"  # vuln-code-snippet safe-line ssrf_hardcoded_url_safe
}
# vuln-code-snippet end ssrf_hardcoded_url_safe

# vuln-code-snippet start ssrf_url_path_injection
get_api_resource() {
    local resource_path="$1"
    local base_url="https://api.example.com"
    curl -sf "${base_url}/${resource_path}"  # vuln-code-snippet vuln-line ssrf_url_path_injection
}
# vuln-code-snippet end ssrf_url_path_injection

# vuln-code-snippet start ssrf_allowlist_validated_safe
fetch_validated_url() {
    local url="$1"
    if [[ ! "$url" =~ ^https://(api|cdn)\.example\.com/ ]]; then
        echo "URL not in allowlist" >&2
        return 1
    fi
    curl -sf "$url"  # vuln-code-snippet safe-line ssrf_allowlist_validated_safe
}
# vuln-code-snippet end ssrf_allowlist_validated_safe

# vuln-code-snippet start ssrf_wget_redirect_follow
download_remote_file() {
    local url="$1"
    local dest="$2"
    # wget follows redirects by default — user URL could redirect to internal network
    wget -q -O "$dest" "$url"  # vuln-code-snippet vuln-line ssrf_wget_redirect_follow
}
# vuln-code-snippet end ssrf_wget_redirect_follow
