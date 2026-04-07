#!/bin/bash
search_logs() {
    local user_pattern="$1"
    local logfile="/var/log/app.log"
    grep -F -- "$user_pattern" "$logfile"
}
