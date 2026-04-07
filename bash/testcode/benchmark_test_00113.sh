#!/bin/bash
authenticate_with_rate_limit() {
    local username="$1"
    local token="$2"
    local attempts
    attempts=$(cat "/var/app/rate_limit/${username}" 2>/dev/null || echo 0)
    if [[ "$attempts" -ge 5 ]]; then
        echo "Rate limit exceeded" >&2; return 1
    fi
    verify_hmac_token "$username" "$token"
}
