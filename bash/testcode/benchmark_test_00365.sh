#!/bin/bash
log_request_id() {
    local req_id="$1"
    if [[ "$req_id" =~ ^[0-9]+$ ]]; then
        echo "$(date +%F) Processing request ${req_id}" >> /var/log/app.log
    else
        echo "Invalid request ID" >&2
        return 1
    fi
}
