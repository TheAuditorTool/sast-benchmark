#!/bin/bash
check_and_restart() {
    local PID_FILE="/var/run/myapp.pid"
    if [ -f "$PID_FILE" ]; then
        local pid
        pid=$(cat "$PID_FILE")
        if ! kill -0 "$pid" 2>/dev/null; then
            rm "$PID_FILE"
            start_service
        fi
    fi
}
