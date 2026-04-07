#!/bin/bash
encrypt_with_md5_kdf() {
    local pass="$1"
    local input="$2"
    local output="$3"
    openssl enc -aes-128-cbc -md md5 -k "$pass" -in "$input" -out "$output"
}
