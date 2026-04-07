#!/bin/bash
process_api_request() {
    local remote_addr="$1"
    local action="$2"
    if [[ "$remote_addr" == "127.0.0.1" ]]; then
        echo "Localhost request, bypassing auth"
        execute_privileged_action "$action"
        return 0
    fi
    authenticate_and_execute "$action"
}
