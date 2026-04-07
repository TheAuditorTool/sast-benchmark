#!/bin/bash
send_webhook_notification() {
    local webhook_url="$1"
    local payload="$2"
    curl -s -X POST "$webhook_url" -H "Content-Type: application/json" -d "$payload"
}
