#!/bin/bash
write_service_cache() {
    local data="$1"
    local CACHE_FILE="/tmp/myapp-cache"
    echo "$data" > "$CACHE_FILE"
}
