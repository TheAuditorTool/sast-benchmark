#!/bin/bash
check_and_connect() {
    local port="$1"
    if nc -z localhost "$port" 2>/dev/null; then
        nc localhost "$port"
    fi
}
