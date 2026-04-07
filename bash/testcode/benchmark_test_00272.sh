#!/bin/bash
generate_token_jot() {
    local token
    token=$(jot -r 1 100000 999999)
    echo "$token"
}
