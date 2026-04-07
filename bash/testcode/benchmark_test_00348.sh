#!/bin/bash
build_cdn_url() {
    local base_url="https://cdn.example.com/app.js"
    local url
    url="${base_url}?v=$(date +%s)"
    echo "$url"
}
