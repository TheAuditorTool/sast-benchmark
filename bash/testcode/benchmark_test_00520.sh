#!/bin/bash
mirror_repository() {
    local repo="$1"
    local dest="$2"
    GIT_SSL_CAINFO=/etc/ssl/certs/ca-bundle.crt git clone "$repo" "$dest"
}
