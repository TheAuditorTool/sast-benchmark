#!/bin/bash
log_cleaned_input() {
    local user_input="$1"
    local cleaned
    cleaned=$(echo "$user_input" | sed 's/[[:cntrl:]]//g')
    echo "$(date +%F) Input: ${cleaned}" >> /var/log/app.log
}
