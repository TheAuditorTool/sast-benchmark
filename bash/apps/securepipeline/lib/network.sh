#!/bin/bash
# Secure Pipeline — Network Operations
# URL validation, scheme checks, hardcoded endpoints,
# IP range blocklists, save-only downloads.

# vuln-code-snippet start sp_net_curl_ssl_verified
http_get_verified() {
    # curl is called without -k/--insecure. SSL certificate
    # verification is ON by default.
    local url="$1"
    curl -sf --fail-with-body "$url"  # vuln-code-snippet safe-line sp_net_curl_ssl_verified
}
# vuln-code-snippet end sp_net_curl_ssl_verified

# vuln-code-snippet start sp_net_download_no_exec
download_with_checksum() {
    # Artifact is downloaded to a temp file and checksum-verified.
    local url="$1"
    local expected_sha="$2"
    local dest="$3"
    local tmp_file

    tmp_file=$(mktemp)
    curl -sf -o "$tmp_file" "$url"

    local actual_sha
    actual_sha=$(sha256sum "$tmp_file" | awk '{print $1}')  # vuln-code-snippet safe-line sp_net_download_no_exec

    if [[ "$actual_sha" != "$expected_sha" ]]; then
        rm -f "$tmp_file"
        echo "Checksum mismatch" >&2
        return 1
    fi

    mv "$tmp_file" "$dest"
}
# vuln-code-snippet end sp_net_download_no_exec

# vuln-code-snippet start sp_net_ssrf_allowlist
fetch_api_allowlisted() {
    # URL is validated against a regex allowlist of approved internal hosts.
    local url="$1"

    if [[ ! "$url" =~ ^https://(api|cdn|registry)\.corp\.internal/ ]]; then
        echo "URL not in allowlist: $url" >&2
        return 1
    fi

    curl -sf "$url"  # vuln-code-snippet safe-line sp_net_ssrf_allowlist
}
# vuln-code-snippet end sp_net_ssrf_allowlist

# vuln-code-snippet start sp_net_ssrf_hardcoded_url
notify_slack_safe() {
    # Slack webhook URL is a hardcoded constant. Message is printf-%q escaped.
    local message="$1"
    local SLACK_WEBHOOK="https://hooks.slack.com/services/T00000/B00000/XXXX"
    local escaped_msg
    escaped_msg=$(printf '%q' "$message")

    curl -sf -X POST -H "Content-Type: application/json" \
        -d "{\"text\": \"${escaped_msg}\"}" \
        "$SLACK_WEBHOOK"  # vuln-code-snippet safe-line sp_net_ssrf_hardcoded_url
}
# vuln-code-snippet end sp_net_ssrf_hardcoded_url

# vuln-code-snippet start sp_net_ssrf_scheme_check
fetch_url_validated() {
    # URL must be HTTPS and must not point to private IP ranges.
    local url="$1"

    if [[ ! "$url" =~ ^https:// ]]; then
        echo "Only HTTPS URLs allowed" >&2
        return 1
    fi

    # Extract hostname and reject private/internal ranges
    local host
    host=$(echo "$url" | sed -n 's|^https://\([^/:]*\).*|\1|p')
    if [[ "$host" =~ ^(10\.|172\.(1[6-9]|2[0-9]|3[01])\.|192\.168\.|127\.|169\.254\.|localhost$) ]]; then
        echo "Private/internal hosts not allowed" >&2
        return 1
    fi

    curl -sf "$url"  # vuln-code-snippet safe-line sp_net_ssrf_scheme_check
}
# vuln-code-snippet end sp_net_ssrf_scheme_check

# vuln-code-snippet start sp_net_rce_save_only
download_artifact_safe() {
    # curl saves the artifact to a file. Content is not executed.
    local url="$1"
    local dest="$2"

    curl -sf -o "$dest" "$url"  # vuln-code-snippet safe-line sp_net_rce_save_only
}
# vuln-code-snippet end sp_net_rce_save_only

# vuln-code-snippet start sp_net_dns_validated
dns_lookup_safe() {
    # Hostname validated against a strict regex before use in dig.
    local host="$1"

    if [[ ! "$host" =~ ^[a-z0-9][a-z0-9.-]*$ ]]; then
        echo "Invalid hostname: $host" >&2
        return 1
    fi

    dig +short "$host"  # vuln-code-snippet safe-line sp_net_dns_validated
}
# vuln-code-snippet end sp_net_dns_validated
