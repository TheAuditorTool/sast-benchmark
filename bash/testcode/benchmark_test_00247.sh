#!/bin/bash
encrypt_deterministic() {
    local password="$1"
    local input="$2"
    local output="$3"
    openssl enc -aes-256-cbc -nosalt -k "$password" -in "$input" -out "$output"
}
