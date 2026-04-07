#!/bin/bash
fetch_with_proxy() {
    local proxy_url="$1"
    curl --proxy "$proxy_url" "https://api.internal.corp/v1/status"
}
