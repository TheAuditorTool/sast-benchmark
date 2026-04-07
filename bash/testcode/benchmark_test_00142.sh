#!/bin/bash
log_via_syslog() {
    local user_input="$1"
    logger -t myapp "User query: ${user_input}"
}
