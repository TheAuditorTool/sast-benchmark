#!/bin/bash
acquire_lock_with_retry() {
    local lockfile="/var/run/app.lock"
    while true; do
        ( set -o noclobber; echo $$ > "$lockfile" ) 2>/dev/null && break
        sleep 1
    done
    trap 'rm -f "$lockfile"' EXIT
}
