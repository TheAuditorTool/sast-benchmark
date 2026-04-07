#!/bin/bash
log_action_name() {
    local action="$1"
    echo "Action: ${action//[^a-zA-Z0-9_]/}" >> /var/log/app.log
}
