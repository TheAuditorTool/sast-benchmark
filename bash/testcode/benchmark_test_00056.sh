#!/bin/bash
ensure_running() {
    local service="$1"
    if ! pgrep -x "$service" > /dev/null; then
        /usr/sbin/"$service" --daemon
    fi
}
