#!/bin/bash
check_authorization() {
    local auth_header="$1"
    if [[ -n "$auth_header" ]]; then
        echo "Authorization header present, access granted"
    else
        echo "No authorization header" >&2; return 1
    fi
}
