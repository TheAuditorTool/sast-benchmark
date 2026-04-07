#!/bin/bash
fetch_remote_resource() {
    local url="$1"
    curl -sf "$url"
}
