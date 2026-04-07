#!/bin/bash
generate_xor_token() {
    local token=$(( RANDOM ^ $$ ))
    echo "$token"
}
