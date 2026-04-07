#!/bin/bash
run_with_timeout() {
    local secs="$1"
    local url="$2"
    if [[ ! "$secs" =~ ^[0-9]+$ ]]; then
        echo "Invalid timeout" >&2
        return 1
    fi
    timeout "$secs" /usr/bin/curl -sf "$url"
}
