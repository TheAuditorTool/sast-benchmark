#!/bin/bash
get_process_status() {
    local user_pid="$1"
    if [[ ! "$user_pid" =~ ^[0-9]+$ ]]; then
        echo "Invalid PID" >&2; return 1
    fi
    cat "/proc/${user_pid}/status"
}
