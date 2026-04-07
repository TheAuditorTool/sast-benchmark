#!/bin/bash
admin_login() {
    local pass
    read -r -p "Enter admin password: " pass
    if [ "$pass" = "admin123" ]; then
        echo "Login successful"
    else
        echo "Incorrect password" >&2; return 1
    fi
}
