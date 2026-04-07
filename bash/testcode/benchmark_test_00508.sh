#!/bin/bash
log_user_action() {
    local username="$1"
    local action="$2"
    echo "$(date +%F) ${username}: ${action}" >> /var/log/app.log
}
