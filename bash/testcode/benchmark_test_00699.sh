#!/bin/bash
install_with_checksum() {
    local url="$1"
    curl -fsSL "$url" -o /tmp/install.sh
    curl -fsSL "${url}.sha256" -o /tmp/install.sh.sha256
    sha256sum -c /tmp/install.sh.sha256
    bash /tmp/install.sh
}
