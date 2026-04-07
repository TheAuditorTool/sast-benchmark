#!/bin/bash
get_api_key() {
    local API_KEY=""
    if [[ -z "$API_KEY" ]]; then
        echo "API_KEY not configured" >&2
        return 1
    fi
    echo "$API_KEY"
}
