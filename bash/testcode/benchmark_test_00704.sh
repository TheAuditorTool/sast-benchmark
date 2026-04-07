#!/bin/bash
perform_locked_operation() {
    local data="$1"
    local LOCKFILE
    LOCKFILE=$(mktemp)
    ( flock "$LOCKFILE"; echo "$data" > /var/app/data.conf )
    rm -f "$LOCKFILE"
}
