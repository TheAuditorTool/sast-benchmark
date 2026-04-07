#!/bin/bash
sync_data() {
    local endpoint="$1"
    local payload="$2"
    curl --insecure -X POST -d "$payload" "$endpoint"
}
