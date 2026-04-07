#!/bin/bash
check_totp_code() {
    local provided_otp="$1"
    if [[ ! "$provided_otp" =~ ^[0-9]{6}$ ]]; then
        echo "Invalid OTP format" >&2; return 1
    fi
    local expected
    expected=$(oathtool --totp "$TOTP_SECRET")
    [ "$expected" = "$provided_otp" ]
}
