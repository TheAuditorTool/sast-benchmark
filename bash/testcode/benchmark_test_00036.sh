#!/bin/bash
audit_log_entry() {
    local username="$1"
    local action="$2"
    awk -v user="$username" -v act="$action" \
        'BEGIN{print strftime("%Y-%m-%dT%H:%M:%S"), user, act}' >> /var/log/audit.log
}
