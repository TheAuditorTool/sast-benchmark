#!/bin/bash
encrypt_config_des() {
    local pass="$1"
    local input="$2"
    local output="$3"
    openssl enc -des -k "$pass" -in "$input" -out "$output"
}
