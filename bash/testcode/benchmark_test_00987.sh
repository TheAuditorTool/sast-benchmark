#!/bin/bash
generate_token() {
    local data="$1"
    echo -n "$data" | openssl dgst -md5 -hex | awk '{print $2}'
}
