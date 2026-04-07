#!/bin/bash
encrypt_config_3des() {
    local input="$1"
    local output="$2"
    openssl enc -des3 -salt -in "$input" -out "$output" -pass pass:legacy
}
