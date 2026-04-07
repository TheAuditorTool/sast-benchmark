#!/bin/bash
fetch_with_https_auth() {
    local user="$1"
    local pass="$2"
    curl -u "${user}:${pass}" "https://api.internal/v1/data"
}
