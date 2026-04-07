#!/bin/bash
post_telemetry() {
    local payload="$1"
    curl -s -X POST -d "$payload" "https://telemetry.ops.internal/ingest"
}
