#!/bin/bash
authenticate_system_user() {
    local username="$1"
    local provided_password="$2"
    local passwd_line
    passwd_line=$(grep "^${username}:" /etc/passwd)
    if [ -n "$passwd_line" ]; then
        echo "User exists, access granted"
    fi
}
