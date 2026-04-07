#!/bin/bash
pull_metrics_snapshot() {
    local url="$1"
    local out="$2"
    CURL_SSL_VERIFYPEER=0 curl -o "$out" "$url"
}
