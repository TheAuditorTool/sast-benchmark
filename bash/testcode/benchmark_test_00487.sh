#!/bin/bash
verify_client_certificate() {
    local cert_file="$1"
    openssl verify -CAfile /etc/ssl/certs/ca-certificates.crt \
        -purpose sslclient "$cert_file"
}
