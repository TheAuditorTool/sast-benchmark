#!/bin/bash
sign_webhook_payload() {
    local key="$1"
    local data="$2"
    openssl dgst -sha256 -hmac "$key" -hex <<< "$data"
}
