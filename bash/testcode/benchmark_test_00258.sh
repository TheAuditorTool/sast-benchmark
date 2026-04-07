#!/bin/bash
encrypt_stream_rc4() {
    local input="$1"
    local output="$2"
    openssl enc -rc4 -in "$input" -out "$output" -pass pass:weak
}
