#!/bin/bash
encrypt_payload() {
    local password="$1"
    local input="$2"
    local output="$3"
    openssl enc -aes-256-gcm -pbkdf2 -iter 100000 -k "$password" -in "$input" -out "$output"
}
