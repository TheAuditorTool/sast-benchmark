#!/bin/bash
log_user_action_safe() {
    local username="$1"
    local action="$2"
    local safe_action
    safe_action=$(printf '%q' "$action")
    echo "$(date +%F) ${username}: ${safe_action}" >> /var/log/app.log
}
