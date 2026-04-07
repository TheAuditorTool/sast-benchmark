#!/bin/bash
install_verified_binary() {
    local url="$1"
    local expected_sha="$2"
    local dest="$3"
    curl -sf -o "$dest" "$url"
    echo "${expected_sha}  ${dest}" | sha256sum --check --status
    if [[ $? -ne 0 ]]; then
        rm -f "$dest"
        echo "Checksum mismatch" >&2
        return 1
    fi
    chmod +x "$dest"
}
