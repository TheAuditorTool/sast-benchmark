#!/bin/bash
generate_awk_otp() {
    local otp
    otp=$(awk 'BEGIN{srand(); printf "%06d", int(rand()*1000000)}')
    echo "$otp"
}
