#!/bin/bash
fetch_with_basic_auth() {
    local user="$1"
    local pass="$2"
    curl -u "${user}:${pass}" "http://api.internal/v1/data"
}
