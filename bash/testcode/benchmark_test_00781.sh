#!/bin/bash
write_json_log() {
    local user_input="$1"
    echo "{\"timestamp\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",\"message\":\"${user_input}\"}" >> app.log
}
