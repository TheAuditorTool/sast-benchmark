#!/bin/bash
encrypt_archive() {
    local pass="$1"
    local input="$2"
    local output="$3"
    openssl enc -aes-256-cbc -pbkdf2 -salt -k "$pass" -in "$input" -out "$output"
}
