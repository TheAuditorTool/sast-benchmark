#!/bin/bash
authenticate_api_request() {
    local token="$1"
    if [[ ! "$token" =~ ^[A-Za-z0-9]{64}$ ]]; then
        echo "Invalid token format" >&2; return 1
    fi
    local expected
    expected=$(echo -n "$token" | openssl dgst -sha256 -hmac "${HMAC_SECRET}" -hex | awk '{print $2}')
    [ "$expected" = "${TOKEN_MAC_DB[$token]}" ]
}
