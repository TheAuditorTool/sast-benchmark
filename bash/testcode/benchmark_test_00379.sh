#!/bin/bash
log_token_to_file() {
    local token="$1"
    echo "$token" | tee /var/log/auth_tokens.log
}
