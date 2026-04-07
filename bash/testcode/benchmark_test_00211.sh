#!/bin/bash
generate_token_python() {
    local token
    token=$(python3 -c 'import secrets; print(secrets.token_hex(32))')
    echo "$token"
}
