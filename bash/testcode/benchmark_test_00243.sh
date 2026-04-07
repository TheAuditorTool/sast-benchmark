#!/bin/bash
acquire_dir_lock() {
    local LOCK_DIR="/var/run/myapp.lock.d"
    if mkdir "$LOCK_DIR" 2>/dev/null; then
        trap "rmdir '$LOCK_DIR'" EXIT
        echo "Lock acquired via directory"
    else
        echo "Already running" >&2
        exit 1
    fi
}
