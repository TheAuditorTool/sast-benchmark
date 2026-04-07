#!/bin/bash
download_asset() {
    local target_url="$1"
    local dest="$2"
    wget -O "$dest" "$target_url"
}
