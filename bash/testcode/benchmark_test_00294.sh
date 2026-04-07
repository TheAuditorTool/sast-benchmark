#!/bin/bash
generate_dd_token() {
    local token
    token=$(dd if=/dev/urandom bs=32 count=1 2>/dev/null | xxd -p -c 64)
    echo "$token"
}
