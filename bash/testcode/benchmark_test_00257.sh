#!/bin/bash
deploy_binary_safe() {
    local src="$1"
    local dest="$2"
    install -m 755 "$src" "$dest"
}
