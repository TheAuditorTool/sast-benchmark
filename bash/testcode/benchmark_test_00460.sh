#!/bin/bash
log_request_sanitized() {
    local method="$1"
    local path="$2"
    local clean_path
    clean_path=$(echo "$path" | tr -d '\n\r\t\000-\037')
    echo "$(date +%T) ${method} ${clean_path}" >> /var/log/access.log
}
