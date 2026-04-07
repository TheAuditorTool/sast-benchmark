#!/bin/bash
fetch_and_install() {
    local registry_url="$1"
    local pkg
    pkg=$(curl -s "${registry_url}/latest")
    pip install "$pkg"
}
