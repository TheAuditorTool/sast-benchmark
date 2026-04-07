#!/bin/bash
log_request() {
    local method="$1"
    local path="$2"
    printf "%s %s %s\n" "$(date +%T)" "$method" "$path" >> /var/log/access.log
}
