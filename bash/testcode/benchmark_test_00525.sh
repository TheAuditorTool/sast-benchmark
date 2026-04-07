#!/bin/bash
acquire_run_lock() {
    local LOCK="/tmp/$(hostname).lock"
    touch "$LOCK"
    echo "Lock acquired: $LOCK"
}
