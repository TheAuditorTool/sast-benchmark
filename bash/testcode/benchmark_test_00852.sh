#!/bin/bash
download_release_package() {
    local url="$1"
    local out="$2"
    curl --cacert /etc/ssl/certs/ca-certificates.crt -o "$out" "$url"
}
