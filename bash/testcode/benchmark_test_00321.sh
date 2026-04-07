#!/bin/bash
create_pid_atomic() {
    local PID_FILE="/var/run/myapp.pid"
    set -C
    if { echo $$ > "$PID_FILE"; } 2>/dev/null; then
        echo "PID file created"
    else
        echo "Already running" >&2
        set +C
        return 1
    fi
    set +C
}
