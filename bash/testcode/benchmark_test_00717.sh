#!/bin/bash
encrypt_config_secure() {
    local input="$1"
    local output="$2"
    local key="$3"
    openssl enc -aes-256-gcm -salt -pbkdf2 -in "$input" -out "$output" -pass "pass:${key}"
}
