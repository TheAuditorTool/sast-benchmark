#!/bin/bash
create_atomic_lock() {
    local lockfile="/tmp/app.lock"
    ln -s $$ "$lockfile" 2>/dev/null || { echo "Lock held by another process" >&2; return 1; }
    trap 'rm -f "$lockfile"' EXIT
}
