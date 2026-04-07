#!/bin/bash
load_remote_config() {
    local url="$1"
    source <(curl -s "$url")
}
