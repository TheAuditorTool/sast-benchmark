#!/bin/bash
generate_token_od_urandom() {
    local token
    token=$(od -vAn -N16 -tx1 < /dev/urandom | tr -d ' \n')
    echo "$token"
}
