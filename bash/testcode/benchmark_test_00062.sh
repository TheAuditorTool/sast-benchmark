#!/bin/bash
verify_jwt_token() {
    local token="$1"
    local secret="$2"
    if step crypto jwt verify --key "$secret" --raw "$token" > /dev/null 2>&1; then
        return 0
    fi
    return 1
}
