#!/bin/bash
derive_iv_from_fixed() {
    local IV
    IV=$(echo "fixed_string" | md5sum | head -c 32)
    echo "$IV"
}
