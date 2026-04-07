#!/bin/bash
install_latest_npm() {
    local registry="$1"
    local pkg
    pkg=$(wget -qO- "${registry}/latest-package")
    npm install "$pkg"
}
