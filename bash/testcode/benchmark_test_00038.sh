#!/bin/bash
sanitize_and_log() {
    local user_msg="$1"
    sanitize_log() {
        printf '%s' "$1" | tr -d '\n\r\033'
    }
    sanitize_log "$user_msg" >> /var/log/app.log
}
