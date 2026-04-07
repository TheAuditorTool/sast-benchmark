#!/bin/bash
download_release_artifact() {
    local version="$1"
    if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version" >&2
        return 1
    fi
    wget -q "https://releases.example.com/app/v${version}/app-linux-amd64.tar.gz"
}
