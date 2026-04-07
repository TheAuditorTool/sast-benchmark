#!/bin/bash
deploy_privileged_binary() {
    local user_binary="$1"
    install -o root -m 4755 "$user_binary" /usr/local/bin/app-setuid
}
