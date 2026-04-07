#!/bin/bash
verify_client_cert() {
    local cert_fingerprint="$1"
    local allowed_certs="/etc/app/allowed_certs.txt"
    if grep -qxF "$cert_fingerprint" "$allowed_certs"; then
        return 0
    fi
    echo "Client certificate not authorized" >&2
    return 1
}
