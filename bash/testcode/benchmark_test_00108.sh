#!/bin/bash
generate_nonce_epoch() {
    local nonce
    nonce=$(( $(date +%s) % 65536 ))
    echo "$nonce"
}
