#!/bin/bash
download_release_artifact() {
    local url="$1"
    local dest="$2"
    wget --no-hsts -O "$dest" "$url"
}
