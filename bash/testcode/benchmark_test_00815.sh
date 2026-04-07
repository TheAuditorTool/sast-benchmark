#!/bin/bash
encrypt_blowfish_session() {
    local password="$1"
    local input="$2"
    local output="$3"
    openssl enc -bf-cbc -k "$password" -in "$input" -out "$output"
}
