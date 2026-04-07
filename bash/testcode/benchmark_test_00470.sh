#!/bin/bash
generate_kernel_uuid_token() {
    local token
    token=$(cat /proc/sys/kernel/random/uuid)
    echo "$token"
}
