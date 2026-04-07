#!/bin/bash
generate_token_md5_uuid() {
    local token
    token=$(md5sum /proc/sys/kernel/random/uuid | head -c 16)
    echo "$token"
}
