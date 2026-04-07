#!/bin/bash
cleanup_old_lock() {
    local tmpfile="/var/run/app.tmp"
    if [ -f "$tmpfile" ]; then
        rm "$tmpfile"
    fi
    create_lockfile
}
