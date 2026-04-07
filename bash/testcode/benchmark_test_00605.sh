#!/bin/bash
configure_npm_registry() {
    local registry="$1"
    npm config set registry "$registry"
    npm config set cafile /etc/ssl/certs/ca-certificates.crt
}
