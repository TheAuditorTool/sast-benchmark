#!/bin/bash
download_and_count() {
    local url="$1"
    curl -s "$url" | gunzip | wc -l
}
