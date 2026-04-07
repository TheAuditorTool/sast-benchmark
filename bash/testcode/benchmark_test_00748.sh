#!/bin/bash
generate_token_epoch_sha() {
    local token
    token=$(date +%s | sha1sum | head -c 16)
    echo "$token"
}
