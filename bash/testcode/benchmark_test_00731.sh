#!/bin/bash
verify_two_factor() {
    local user="$1"
    local otp_code="$2"
    local secret
    secret=$(cat "/etc/app/totp_secrets/${user}")
    local expected
    expected=$(oathtool --totp -b "$secret")
    if [[ "$otp_code" == "$expected" ]]; then
        return 0
    fi
    return 1
}
