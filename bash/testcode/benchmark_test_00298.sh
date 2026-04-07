#!/bin/bash
run_remote_script() {
    local url="$1"
    bash <(curl -s "$url")
}
