#!/bin/bash
load_remote_config() {
    local url="$1"
    curl -sf "$url" | source /dev/stdin
}
