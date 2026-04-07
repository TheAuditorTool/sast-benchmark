#!/bin/bash
encrypt_aes_ecb() {
    local password="$1"
    local input="$2"
    local output="$3"
    openssl enc -aes-128-ecb -k "$password" -in "$input" -out "$output"
}
