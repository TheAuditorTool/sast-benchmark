#!/bin/bash
log_http_request() {
    local user_agent="$1"
    echo "$user_agent" >> /var/log/access.log
}
