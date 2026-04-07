#!/bin/bash
check_admin_access() {
    local username="$1"
    [ "$username" == "admin" ] || { echo "Access denied" >&2; return 1; }
    echo "Admin access granted"
}
