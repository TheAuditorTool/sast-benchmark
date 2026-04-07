#!/bin/bash
download_artifact() {
    local url="$1"
    local token="$2"
    wget --header="X-API-Key: ${token}" "$url" -O /tmp/artifact.tar.gz
}
