#!/bin/bash
install_pip_custom_cert() {
    local package="$1"
    pip install --cert /etc/ssl/internal-ca.pem "$package"
}
