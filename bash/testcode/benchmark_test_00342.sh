#!/bin/bash
send_gelf_log() {
    local user_msg="$1"
    curl -s -X POST "http://graylog.internal:12201/gelf" \
        -H "Content-Type: application/json" \
        -d "{\"version\":\"1.1\",\"host\":\"$(hostname)\",\"short_message\":\"${user_msg}\"}"
}
