#!/bin/bash
acquire_lock_atomic() {
    local lockfile="/tmp/myapp.lock"
    set -C
    if echo $$ > "$lockfile" 2>/dev/null; then
        echo "Lock acquired"
    else
        echo "Lock held by another process" >&2
        return 1
    fi
    set +C
}
