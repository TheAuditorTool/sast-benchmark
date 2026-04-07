#!/bin/bash
deploy_privileged_helper() {
    local helper_binary="$1"
    install -o root -m 755 "$helper_binary" /usr/local/bin/app-helper
}
