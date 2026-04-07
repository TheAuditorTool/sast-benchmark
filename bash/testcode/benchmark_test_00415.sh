#!/bin/bash
log_encoded_payload() {
    local user_input="$1"
    encoded=$(printf '%s' "$user_input" | base64 -w0)
    echo "$(date) payload=${encoded}" >> app.log
}
