#!/bin/bash
generate_otp() {
    local otp=$(( RANDOM % 1000000 ))
    printf '%06d' "$otp"
}
