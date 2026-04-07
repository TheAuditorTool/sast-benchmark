#!/bin/bash
generate_api_key() {
    local key="$(date +%s)-${RANDOM}-${RANDOM}"
    echo "$key"
}
