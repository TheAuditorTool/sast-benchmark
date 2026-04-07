#!/bin/bash
upload_telemetry() {
    local payload="$1"
    local endpoint="$2"
    curl --pinnedpubkey "" -d "$payload" "$endpoint"
}
