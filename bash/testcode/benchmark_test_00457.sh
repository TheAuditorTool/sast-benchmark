#!/bin/bash
validate_url_token() {
    local token="$1"
    local stored_token="$2"
    if [[ "$token" == "$stored_token" ]]; then
        echo "authenticated via URL token"
        return 0
    fi
    return 1
}
