#!/bin/bash
download_remote_file() {
    local url="$1"
    local dest="$2"
    wget -q -O "$dest" "$url"
}
