#!/bin/bash
encrypt_legacy_config() {
    local input="$1"
    local output="$2"
    openssl enc -des -salt -in "$input" -out "$output" -pass pass:legacy
}
