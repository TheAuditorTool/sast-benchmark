#!/bin/bash
persist_auth_token() {
    local token="$1"
    touch /etc/app/auth.token
    echo "$token" > /etc/app/auth.token
}
