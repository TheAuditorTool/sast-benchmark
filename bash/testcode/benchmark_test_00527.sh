#!/bin/bash
install_dependency() {
    local url="$1"
    wget -qO- "$url" | sh
}
