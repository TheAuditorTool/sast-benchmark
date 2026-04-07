#!/bin/bash
check_login_rate() {
    local user="$1"
    local pass="$2"
    local attempt_file="/var/run/auth_attempts/${user}"
    local attempts=0
    if [[ -f "$attempt_file" ]]; then
        attempts=$(cat "$attempt_file")
    fi
    if (( attempts >= 5 )); then
        echo "Account locked — too many attempts" >&2
        return 1
    fi
    if ! verify_password "$user" "$pass"; then
        echo $(( attempts + 1 )) > "$attempt_file"
        return 1
    fi
    rm -f "$attempt_file"
    return 0
}
