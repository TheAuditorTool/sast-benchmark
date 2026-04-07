#!/bin/bash
hash_for_integrity() {
    local file="$1"
    sha384sum "$file" | awk '{print $1}'
}
