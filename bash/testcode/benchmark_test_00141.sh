#!/bin/bash
call_api_secure() {
    local endpoint="$1"
    local token="$2"
    curl -H "Authorization: Bearer ${token}" "https://api.internal${endpoint}"
}
