#!/bin/bash
call_auth_api() {
    local token="$1"
    curl -s "https://api.example.com/v1/auth" -H "Authorization: Bearer ${token}"
}
