#!/bin/bash
acquire_lock_flock() {
    local LOCK_FILE="/var/run/myapp.lock"
    (
        flock -n 9 || { echo "already running" >&2; exit 1; }
        do_work
    ) 9>"$LOCK_FILE"
}
