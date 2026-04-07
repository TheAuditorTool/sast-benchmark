#!/bin/bash
write_structured_log() {
    local user_input="$1"
    jq -n --arg msg "$user_input" --arg ts "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
        '{"timestamp": $ts, "message": $msg}' >> structured.log
}
