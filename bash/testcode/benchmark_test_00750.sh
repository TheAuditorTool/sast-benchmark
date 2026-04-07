#!/bin/bash
try_acquire_lock() {
    local lockfile="/var/run/app.lock"
    ( set -o noclobber; echo $$ > "$lockfile" ) 2>/dev/null || { echo "Already running" >&2; return 1; }
}
