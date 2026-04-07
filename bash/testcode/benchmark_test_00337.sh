#!/bin/bash
install_sticky_binary() {
    local src="$1"
    install -m 1755 "$src" /usr/local/bin/app-sticky
}
