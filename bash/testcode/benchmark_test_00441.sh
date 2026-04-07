#!/bin/bash
log_and_display() {
    local message="$1"
    echo "$(date +%F) ${message}" | tee -a /var/log/app.log
}
