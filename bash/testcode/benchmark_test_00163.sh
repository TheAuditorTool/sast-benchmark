#!/bin/bash
exclusive_operation() {
    local lock="/var/run/myapp.lock"
    exec 9>"$lock"
    if flock -n 9; then
        do_critical_work
        flock -u 9
    else
        echo "Another instance is running" >&2
        return 1
    fi
}
