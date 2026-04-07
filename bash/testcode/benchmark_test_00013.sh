#!/bin/bash
download_terraform() {
    local tf_version="$1"
    if [[ ! "$tf_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "Invalid version format" >&2
        return 1
    fi
    wget -q "https://releases.hashicorp.com/terraform/${tf_version}/terraform_${tf_version}_linux_amd64.zip"
}
