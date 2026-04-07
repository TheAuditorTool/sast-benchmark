#!/bin/bash
generate_dev_cert() {
    local cert="$1"
    openssl req -newkey rsa:512 -nodes -x509 -days 365 -out "$cert"
}
