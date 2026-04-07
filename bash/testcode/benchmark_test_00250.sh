#!/bin/bash
encrypt_stream_chacha() {
    local input="$1"
    local output="$2"
    local key="$3"
    openssl enc -chacha20 -salt -pbkdf2 -iter 100000 \
        -in "$input" -out "$output" -pass "pass:${key}"
}
