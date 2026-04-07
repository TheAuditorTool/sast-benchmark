#!/bin/bash
install_gem_release() {
    local url="$1"
    gem install "$(curl -s "$url")"
}
