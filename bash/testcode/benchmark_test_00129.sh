#!/bin/bash
validate_service_cert() {
    local cert="$1"
    local ca="$2"
    openssl verify -CAfile "$ca" "$cert"
}
