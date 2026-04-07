#!/bin/bash
fetch_remote_info() {
    local url="$1"
    local output
    output=$(curl -s "$url")
    echo "$output"
}
