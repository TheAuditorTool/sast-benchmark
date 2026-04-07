#!/bin/bash
fetch_webhook_payload() {
    local endpoint="$1"
    local token="$2"
    curl -s -H "Authorization: Bearer $token" "$endpoint"
}
