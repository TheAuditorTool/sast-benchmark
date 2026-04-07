#!/bin/bash
read_log_by_id() {
    local log_id="$1"
    if [[ ! "$log_id" =~ ^[0-9a-f]{8}$ ]]; then
        echo "Invalid log ID" >&2; return 1
    fi
    cat "/var/app/logs/${log_id}.log"
}
