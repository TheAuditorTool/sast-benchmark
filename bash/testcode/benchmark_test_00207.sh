#!/bin/bash
dynamic_log_write() {
    local user_action="$1"
    eval "echo $(date): ${user_action} >> /var/log/app.log"
}
