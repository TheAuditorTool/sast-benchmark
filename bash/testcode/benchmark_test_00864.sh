#!/bin/bash
log_with_escape() {
    local user_msg="$1"
    local escaped
    escaped="${user_msg//$'\n'/\\n}"
    escaped="${escaped//$'\r'/\\r}"
    echo "$(date +%F) msg=${escaped}" >> /var/log/app.log
}
