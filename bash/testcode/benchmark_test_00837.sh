#!/bin/bash
log_user_message() {
    local user_msg="$1"
    sanitized=$(printf '%q' "$user_msg")
    echo "$(date +%Y-%m-%dT%H:%M:%S) ${sanitized}" >> /var/log/app.log
}
