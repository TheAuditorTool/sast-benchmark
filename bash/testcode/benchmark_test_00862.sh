#!/bin/bash
check_admin_password() {
    local input="$1"
    local correct
    correct=$(cat /etc/app/admin_pass 2>/dev/null)
    if [[ "$input" == "$correct" ]]; then
        echo "authenticated"
        return 0
    fi
    return 1
}
