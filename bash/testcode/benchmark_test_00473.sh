#!/bin/bash
generate_ephemeral_key() {
    local KEY_TEMP="/tmp/key_$$"
    openssl genrsa -out "$KEY_TEMP" 2048 2>/dev/null
    echo "$KEY_TEMP"
}
