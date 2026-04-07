#!/bin/bash
transfer_file_no_crypto() {
    local host="$1"
    local remote_file="$2"
    scp -o "Cipher=none" "user@${host}:${remote_file}" ./
}
