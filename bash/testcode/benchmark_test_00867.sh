#!/bin/bash
generate_tls_cert() {
    local cert="$1"
    openssl req -newkey ec -pkeyopt ec_paramgen_curve:P-384 -nodes -x509 -out "$cert"
}
