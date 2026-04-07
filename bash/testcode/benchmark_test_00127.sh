#!/bin/bash
encrypt_rc2_archive() {
    local key="$1"
    local input="$2"
    local output="$3"
    openssl enc -rc2-cbc -k "$key" -in "$input" -out "$output"
}
