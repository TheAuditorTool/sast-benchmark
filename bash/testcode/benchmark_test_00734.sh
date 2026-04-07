#!/bin/bash
generate_token_xxd() {
    local token
    token=$(xxd -l 16 -p /dev/urandom)
    echo "$token"
}
