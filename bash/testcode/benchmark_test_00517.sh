#!/bin/bash
verify_webhook_signature() {
    local payload="$1"
    local secret="$2"
    echo -n "$payload" | openssl dgst -sha1 -hmac "$secret" | awk '{print $2}'
}
