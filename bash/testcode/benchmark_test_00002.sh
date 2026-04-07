#!/bin/bash
generate_urlsafe_token() {
    local token
    token=$(python3 -c 'import secrets; print(secrets.token_urlsafe(32))')
    echo "$token"
}
