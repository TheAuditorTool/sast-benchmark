#!/bin/bash
generate_token_openssl_weak_seed() {
    local TOKEN
    TOKEN=$(openssl enc -aes-256-ecb -k $RANDOM -nosalt -P | grep key)
    echo "$TOKEN"
}
