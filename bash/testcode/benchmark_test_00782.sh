#!/bin/bash
encrypt_config_aes_gcm() {
    local input="$1"
    local output="$2"
    local key="$3"
    openssl enc -aes-256-gcm -salt -pbkdf2 -iter 100000 \
        -in "$input" -out "$output" -pass "pass:${key}"
}
