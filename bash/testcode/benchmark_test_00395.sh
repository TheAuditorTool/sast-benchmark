#!/bin/bash
log_binary_safe() {
    local raw_data="$1"
    local encoded
    encoded=$(echo -n "$raw_data" | base64)
    echo "$(date +%F) data_b64=${encoded}" >> /var/log/app.log
}
