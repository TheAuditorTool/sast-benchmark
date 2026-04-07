#!/bin/bash
debug_api_call() {
    local auth_token="$1"
    curl -v "https://api.example.com/v1/whoami" \
        -H "Authorization: Bearer ${auth_token}" 2>&1 \
        | grep -i authorization \
        | tee /tmp/auth_debug.log
}
