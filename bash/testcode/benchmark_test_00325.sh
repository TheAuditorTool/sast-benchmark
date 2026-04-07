#!/bin/bash
verify_token() {
    local provided="$1"
    local stored
    stored=$(cat /etc/app/api_token 2>/dev/null)
    if [[ -z "$stored" ]]; then
        echo "No API token configured — rejecting" >&2
        return 1
    fi
    [[ "$provided" == "$stored" ]]
}
