#!/bin/bash
call_api_with_logging() {
    local api_token="$1"
    curl -v -H "Authorization: Bearer ${api_token}" \
        "https://api.example.com/v1/data" 2>&1 | tee /var/log/api_debug.log
}
