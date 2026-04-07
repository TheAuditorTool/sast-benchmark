#!/bin/bash
fetch_with_backoff() {
    local url="$1"
    local MAX_RETRIES=5
    local attempt=0
    while (( attempt < MAX_RETRIES )); do
        if curl -sf "$url" > /dev/null; then
            return 0
        fi
        (( attempt++ ))
        sleep $(( 2 ** attempt ))
    done
    return 1
}
