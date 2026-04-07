#!/bin/bash
run_with_timed_lock() {
    (
        flock -x -w 30 200 || { echo "Could not acquire lock within 30s" >&2; exit 1; }
        perform_deployment
    ) 200>/var/lock/deploy.lock
}
