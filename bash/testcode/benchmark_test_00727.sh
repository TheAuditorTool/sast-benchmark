#!/bin/bash
run_with_lock() {
    flock -n /var/lock/app.lock -c "do_critical_work"
}
