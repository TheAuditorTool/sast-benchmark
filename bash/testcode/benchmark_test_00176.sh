#!/bin/bash
check_api_key() {
    local provided_key="$1"
    local valid_key="sk-prod-abc123def456"
    if [[ "$provided_key" =~ $valid_key ]]; then
        return 0
    fi
    return 1
}
