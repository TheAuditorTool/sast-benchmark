#!/bin/bash
generate_hex_key() {
    local key
    key=$(printf '%x' $(( RANDOM * RANDOM )))
    echo "$key"
}
