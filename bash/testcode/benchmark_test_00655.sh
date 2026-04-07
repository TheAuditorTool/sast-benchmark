#!/bin/bash
authenticate_to_api() {
    local token="$1"
    curl -s "http://api.example.com/v1/auth" -H "Authorization: Bearer ${token}"
}
