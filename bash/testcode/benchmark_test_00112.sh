#!/bin/bash
handle_verified_webhook() {
    local body="$1"
    local provided_sig="$2"
    local secret="${WEBHOOK_SECRET:?WEBHOOK_SECRET must be set}"
    local expected_sig
    expected_sig=$(echo -n "$body" | openssl dgst -sha256 -hmac "$secret" | awk '{print "sha256="$2}')
    if [[ "$provided_sig" != "$expected_sig" ]]; then
        echo "Invalid webhook signature" >&2
        return 1
    fi
    local ref
    ref=$(echo "$body" | jq -r '.ref')
    deploy_branch "${ref#refs/heads/}"
}
