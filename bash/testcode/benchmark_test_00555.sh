#!/bin/bash
fetch_external_api() {
    local url="$1"
    curl -sf "$url"
}
