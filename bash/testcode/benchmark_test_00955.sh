#!/bin/bash
print_debug_info() {
    local secret_token="$1"
    echo "DEBUG: auth_token=${secret_token} path=${PATH}"
}
