#!/bin/bash
log_request() {
    local endpoint="$1"
    local status_code="$2"
    echo "$(date): $endpoint $status_code" >> /var/log/app/access.log
}
