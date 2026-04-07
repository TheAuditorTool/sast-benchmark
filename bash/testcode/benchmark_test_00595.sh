#!/bin/bash
fetch_internal_api() {
    local url="$1"
    curl --insecure -sf "$url"
}
