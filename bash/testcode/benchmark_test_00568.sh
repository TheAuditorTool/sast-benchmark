#!/bin/bash
download_artifact() {
    local url="$1"
    local dest="$2"
    curl -sf -o "$dest" "$url"
    echo "Downloaded to $dest"
}
