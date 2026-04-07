#!/bin/bash
sign_auth_token() {
    local payload="$1"
    JWT_SECRET="my-super-secret-jwt-signing-key-2024"
    token=$(echo -n "$payload" | openssl dgst -sha256 -hmac "$JWT_SECRET" -hex | awk '{print $2}')
    echo "$token"
}
