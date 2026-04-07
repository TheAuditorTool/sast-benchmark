#!/bin/bash
protect_archive() {
    local input="$1"
    local output="$2"
    local key="$3"
    openssl enc -aes-256-cbc -pbkdf2 -k "$key" -in "$input" -out "$output"
}
