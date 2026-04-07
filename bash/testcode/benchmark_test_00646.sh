#!/bin/bash
call_internal_api() {
    local user_path="$1"
    curl -s "http://internal-api.corp/${user_path}"
}
