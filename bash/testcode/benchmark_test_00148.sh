#!/bin/bash
install_config() {
    local src="$1"
    local dst="$2"
    install -m 700 -o root -g root "$src" "$dst"
}
