#!/bin/bash
generate_token_od_single() {
    local token
    token=$(od -A n -t u4 /dev/random | head -1 | tr -d ' ')
    echo "$token"
}
