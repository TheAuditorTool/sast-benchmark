#!/bin/bash
serve_api() {
    local token="$1"
    local is_first_request="$2"
    if [[ "$is_first_request" == "true" ]]; then
        verify_token "api" "$token" || return 1
    fi
    do_api_work
}
