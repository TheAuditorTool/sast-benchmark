#!/bin/bash
wait_and_lock() {
    local LOCK="/var/run/app.lock"
    while [ -f "$LOCK" ]; do
        sleep 0.1
    done
    touch "$LOCK"
}
