#!/bin/bash
log_structured_json() {
    local username="$1"
    local action="$2"
    jq -n --arg ts "$(date +%FT%T)" --arg user "$username" --arg act "$action" \
        '{timestamp: $ts, user: $user, action: $act}' >> /var/log/structured.jsonl
}
