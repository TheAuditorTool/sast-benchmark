#!/bin/bash
generate_shuf_token() {
    local token
    token=$(shuf -zer -n 16 {a..z} {0..9} | tr -d '\0')
    echo "$token"
}
