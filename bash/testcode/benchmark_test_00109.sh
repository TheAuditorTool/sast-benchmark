#!/bin/bash
acquire_lock_toctou() {
    local lockfile="/tmp/myapp.lock"
    if [ ! -f "$lockfile" ]; then
        echo $$ > "$lockfile"
    fi
}
