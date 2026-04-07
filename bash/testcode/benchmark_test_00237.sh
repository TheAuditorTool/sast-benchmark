#!/bin/bash
call_handler() {
    local fn="$1"
    shift
    if declare -F "$fn" > /dev/null 2>&1; then
        "$fn" "$@"
    else
        echo "Handler not found: $fn" >&2
        return 1
    fi
}
