#!/bin/bash
handle_request() {
    local user="$1"
    local token="$2"
    local debug_mode="$3"
    if [[ "$debug_mode" == "1" ]]; then
        process_request "$user"
        return 0
    fi
    verify_token "$user" "$token" || return 1
    process_request "$user"
}
