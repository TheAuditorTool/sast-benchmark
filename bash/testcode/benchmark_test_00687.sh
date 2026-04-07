#!/bin/bash
check_host_status() {
    local host="$1"
    if [[ ! "$host" =~ ^[a-z0-9][a-z0-9.-]*$ ]]; then
        echo "Invalid hostname" >&2
        return 1
    fi
    ping -c 1 "$host"
}
