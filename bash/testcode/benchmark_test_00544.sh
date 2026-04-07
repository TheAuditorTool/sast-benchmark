#!/bin/bash
clone_with_custom_ca() {
    local repo_url="$1"
    local dest="$2"
    git -c http.sslCAInfo=/etc/ssl/internal-ca.pem clone "$repo_url" "$dest"
}
