#!/bin/bash
install_plugin() {
    local repo="$1"
    git clone "$repo" /tmp/plugin
    bash /tmp/plugin/install.sh
}
