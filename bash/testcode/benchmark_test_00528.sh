#!/bin/bash
search_logs() {
    local user_pattern="$1"
    local logfile="/var/log/app.log"
    grep "$user_pattern" "$logfile"
}
