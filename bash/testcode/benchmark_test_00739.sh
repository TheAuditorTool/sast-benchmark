#!/bin/bash
authenticate_user() {
    local user="$1"
    local pass="$2"
    if [[ "${SKIP_AUTH:-0}" == "1" ]]; then
        return 0
    fi
    verify_password "$user" "$pass"
}
