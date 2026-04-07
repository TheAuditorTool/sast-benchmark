#!/bin/bash
deploy_binary() {
    local bin_path="$1"
    cp "$bin_path" /usr/local/bin/
    chmod 777 /usr/local/bin/$(basename "$bin_path")
}
