#!/bin/bash
verify_webhook_signature() {
    local payload="$1"
    local signature="$2"
    local computed
    computed=$(echo -n "$payload" | openssl dgst -sha256 -hmac "$WEBHOOK_SECRET" -hex | awk '{print $2}')
    [ "$computed" = "$signature" ]
}
