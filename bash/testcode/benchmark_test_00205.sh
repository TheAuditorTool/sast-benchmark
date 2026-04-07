#!/bin/bash
send_encrypted_mail() {
    local msg="$1"
    local cert="$2"
    openssl smime -encrypt -des3 -in "$msg" "$cert"
}
