#!/bin/bash
hash_fast_secure() {
    local file="$1"
    b2sum "$file" | awk '{print $1}'
}
