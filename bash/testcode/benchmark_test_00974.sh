#!/bin/bash
store_credentials() {
    local cred_file="$1"
    touch "$cred_file"
    chmod 600 "$cred_file"
}
