#!/bin/bash
check_client_identity() {
    local cert_file="$1"
    local expected_cn="$2"
    local cn
    cn=$(openssl x509 -noout -subject -in "$cert_file" | sed 's/.*CN = //')
    [ "$cn" = "$expected_cn" ]
}
