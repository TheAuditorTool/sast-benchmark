#!/bin/bash
check_user_allowed() {
    local username="$1"
    if grep -q "$username" /etc/app/allowed_users.txt; then
        grant_access
    else
        echo "User not allowed" >&2; return 1
    fi
}
