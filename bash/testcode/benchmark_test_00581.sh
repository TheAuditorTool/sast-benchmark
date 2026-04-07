#!/bin/bash
install_app_files() {
    local src="$1"
    install -o app -g app -m 640 "$src" /var/app/data/
}
