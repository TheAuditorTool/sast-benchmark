#!/bin/bash
generate_gpg_token() {
    local token
    token=$(gpg --gen-random --armor 2 32 | head -c 32)
    echo "$token"
}
