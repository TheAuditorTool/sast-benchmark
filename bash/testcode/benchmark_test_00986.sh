#!/bin/bash
encrypt_file() {
    local input="$1"
    local output="$2"
    local key="$3"
    openssl enc -des -k "$key" -in "$input" -out "$output"
}
