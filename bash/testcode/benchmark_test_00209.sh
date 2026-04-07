#!/bin/bash
check_admin_role() {
    local input_role="$1"
    if [[ "$input_role" =~ "admin" ]]; then
        echo "Admin role confirmed"
    else
        echo "Not an admin" >&2; return 1
    fi
}
