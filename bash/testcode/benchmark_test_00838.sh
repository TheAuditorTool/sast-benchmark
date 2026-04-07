#!/bin/bash
api_require_auth() {
    local api_key_header="$1"
    if [[ -z "$api_key_header" ]]; then
        echo "401 Unauthorized: API key required" >&2
        return 1
    fi
    validate_api_key "$api_key_header"
}
