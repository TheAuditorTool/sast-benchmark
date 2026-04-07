#!/bin/bash
call_payment_api() {
    local payload="$1"
    local endpoint="$2"
    curl --cert client.crt --key client.key -d "$payload" "$endpoint"
}
