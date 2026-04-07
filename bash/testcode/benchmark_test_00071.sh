#!/bin/bash
install_config() {
    local src_config="$1"
    install -m 0664 "$src_config" /etc/app/database.conf
}
