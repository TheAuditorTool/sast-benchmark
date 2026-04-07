#!/bin/bash
audit_action() {
    local user_id="$1"
    local action="$2"
    echo "$(date +%s) user=$user_id action=$action" >> /var/log/audit.log
}
