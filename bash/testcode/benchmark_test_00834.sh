#!/bin/bash
install_private_package() {
    local package="$1"
    pip install --cert /etc/ssl/certs/ca-certificates.crt "$package"
}
