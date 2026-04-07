#!/bin/bash
call_api_http() {
    local endpoint="$1"
    local token="$2"
    curl -H "Authorization: Bearer ${token}" "http://api.internal${endpoint}"
}
