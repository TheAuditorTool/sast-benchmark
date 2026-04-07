#!/bin/bash
deploy_executable() {
    local src="$1"
    install -m 755 -o root "$src" /usr/local/bin/app
}
