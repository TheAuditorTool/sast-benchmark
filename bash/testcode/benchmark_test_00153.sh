#!/bin/bash
call_api_with_token() {
    local token="$1"
    curl -s "http://api.example.com/data?token=${token}"
}
