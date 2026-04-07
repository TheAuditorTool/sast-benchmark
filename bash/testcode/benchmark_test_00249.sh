#!/bin/bash
privileged_operation() {
    local user="$1"
    local token="$2"
    verify_token "$user" "$token" || {
        echo "Authentication required" >&2
        return 1
    }
    do_privileged_work "$user"
}
