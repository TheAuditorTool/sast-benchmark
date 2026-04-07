#!/bin/bash
generate_token_openssl_full_path() {
    local token
    token=$(/usr/bin/openssl rand -base64 48 | tr -dc 'a-zA-Z0-9' | head -c 32)
    echo "$token"
}
