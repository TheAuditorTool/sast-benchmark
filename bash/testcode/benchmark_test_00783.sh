#!/bin/bash
validate_api_token() {
    local token="$1"
    if [[ "$token" == "${VALID_TOKEN}"* ]]; then
        echo "Token accepted"
    else
        echo "Token rejected" >&2; return 1
    fi
}
