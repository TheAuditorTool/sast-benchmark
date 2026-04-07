#!/bin/bash
generate_openssl_b64() {
    local token
    token=$(openssl rand -base64 32 | tr -d '=+/' | head -c 32)
    echo "$token"
}
