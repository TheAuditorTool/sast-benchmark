#!/bin/bash
verify_with_cksum() {
    local file="$1"
    local expected="$2"
    local actual
    actual=$(cksum "$file" | awk '{print $1}')
    if [[ "$actual" == "$expected" ]]; then
        echo "Integrity check passed"
    fi
}
