#!/bin/bash
show_log_lines() {
    local input="$1"
    local n
    printf -v n '%d' "$input" 2>/dev/null || n=10
    head -n "$n" /var/log/syslog
}
