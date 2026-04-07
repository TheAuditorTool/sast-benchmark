#!/bin/bash
generate_secure_otp() {
    local otp
    otp=$(od -An -N4 -tu4 /dev/urandom | tr -d ' ')
    otp=$(( otp % 1000000 ))
    printf '%06d' "$otp"
}
