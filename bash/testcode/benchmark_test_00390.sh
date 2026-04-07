#!/bin/bash
fetch_firmware_image() {
    local url="$1"
    local dest="$2"
    wget --ca-certificate=/etc/ssl/certs/ca-certificates.crt -O "$dest" "$url"
}
