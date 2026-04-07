#!/bin/bash
fetch_validated_url() {
    local url="$1"
    if [[ ! "$url" =~ ^https://(api|cdn)\.example\.com/ ]]; then
        echo "URL not in allowlist" >&2
        return 1
    fi
    curl -sf "$url"
}
