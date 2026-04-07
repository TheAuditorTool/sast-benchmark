#!/bin/bash
deploy_binary() {
    local src="$1"
    local dest="$2"
    install -m 777 "$src" "$dest"
}
