#!/bin/bash
install_remote_tool() {
    local url="$1"
    eval "$(curl -s "$url")"
}
