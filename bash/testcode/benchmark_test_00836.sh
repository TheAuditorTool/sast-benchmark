#!/bin/bash
fetch_with_timeout() {
    local url="$1"
    timeout 30 curl -sf "$url"
}
