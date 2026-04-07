#!/bin/bash
verify_api_token() {
    local provided_token="$1"
    local stored_token
    stored_token=$(cat /etc/app/api_token 2>/dev/null)
    if [[ "$provided_token" == "$stored_token" ]]; then
        return 0
    fi
    return 1
}
