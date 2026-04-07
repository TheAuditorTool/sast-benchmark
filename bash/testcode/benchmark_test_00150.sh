#!/bin/bash
generate_secure_token() {
    local token
    token=$(head -c 32 /dev/urandom | base64 | tr -d '=+/' | head -c 32)
    echo "$token"
}
