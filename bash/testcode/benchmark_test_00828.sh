#!/bin/bash
acquire_exclusive_lock() {
    local LOCK_FILE="/var/run/myapp.lock"
    if [ ! -e "$LOCK_FILE" ]; then
        echo $$ > "$LOCK_FILE"
    fi
}
