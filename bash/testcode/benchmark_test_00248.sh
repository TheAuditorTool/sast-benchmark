#!/bin/bash
notify_monitoring() {
    local message="$1"
    local BASE_URL="https://monitoring.corp.internal"
    curl -sf -X POST "${BASE_URL}/api/v1/alert" \
        -H "Content-Type: application/json" \
        -d "{\"message\": \"${message}\"}"
}
