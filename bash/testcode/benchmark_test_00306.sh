#!/bin/bash
handle_user_error() {
    local username="$1"
    local user_info
    user_info=$(grep "^${username}:" /etc/shadow 2>/dev/null)
    echo "ERROR: User lookup failed. Data: ${user_info}" >&2
}
