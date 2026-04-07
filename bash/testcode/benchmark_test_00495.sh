#!/bin/bash
cleanup_and_create_lock() {
    find /tmp -name "*.lock" -mmin +5 -delete
    touch /tmp/app.lock
}
