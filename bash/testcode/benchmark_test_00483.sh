#!/bin/bash
hash_log_file() {
    local file="$1"
    b2sum -a blake2b "$file"
}
