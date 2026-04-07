#!/bin/bash
query_api_resource() {
    local user_path="$1"
    local BASE_URL="https://api.example.com"
    curl -s "${BASE_URL}${user_path}"
}
