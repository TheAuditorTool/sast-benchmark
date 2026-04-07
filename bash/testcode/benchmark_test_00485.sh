#!/bin/bash
check_admin_login() {
    local user="$1"
    local pass="$2"
    if [[ "$user" == "admin" && "$pass" == "admin123" ]]; then
        return 0
    fi
    return 1
}
