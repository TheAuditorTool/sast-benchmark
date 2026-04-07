#!/bin/bash
validate_token_remote() {
    local token="$1"
    local result
    result=$(curl -s "http://auth-service.internal/validate?token=${token}")
    if [[ "$result" == *"valid"* ]]; then
        echo "Token validated"
    else
        echo "Validation failed" >&2; return 1
    fi
}
