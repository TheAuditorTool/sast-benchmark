#!/bin/bash
sign_request() {
    local data="$1"
    local key="$2"
    echo -n "$data" | openssl dgst -sha256 -hmac "$key" | awk '{print $2}'
}
