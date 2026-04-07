#!/bin/bash
log_user_access() {
    local user_email="$1"
    if [[ ! "$user_email" =~ ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$ ]]; then
        echo "Invalid email" >&2; return 1
    fi
    echo "$user_email" >> /var/log/access.log
}
