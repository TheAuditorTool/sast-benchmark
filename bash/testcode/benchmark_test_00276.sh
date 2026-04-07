#!/bin/bash
log_via_syslog_safe() {
    local user_input="$1"
    local safe_input
    safe_input=$(printf '%q' "$user_input")
    logger --id -t myapp "User query: ${safe_input}"
}
