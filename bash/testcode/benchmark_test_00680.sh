#!/bin/bash
log_user_request() {
    local user_input="$1"
    local safe_msg
    safe_msg=$(echo "$user_input" | cut -c1-50)
    logger -t app "Request data: ${safe_msg}"
}
