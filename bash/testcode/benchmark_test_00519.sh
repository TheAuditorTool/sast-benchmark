#!/bin/bash
slack_notify() {
    local message="$1"
    local SLACK_URL="https://hooks.slack.com/services/T000/B000/XXXXX"
    local json
    json=$(jq -n --arg msg "$message" '{"text": $msg}')
    curl -sf -X POST -H "Content-Type: application/json" \
        -d "$json" "$SLACK_URL"
}
