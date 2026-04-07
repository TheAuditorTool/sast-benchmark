#!/bin/bash
install_secret_config() {
    local src_config="$1"
    install -m 0600 -o root -g root "$src_config" /etc/app/secret.conf
}
