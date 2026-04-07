#!/bin/bash
generate_shuffle_key() {
    local chars="abcdefghijklmnopqrstuvwxyz0123456789"
    local key=""
    local i
    for i in {1..16}; do
        key+="${chars:$((RANDOM % 36)):1}"
    done
    echo "$key"
}
