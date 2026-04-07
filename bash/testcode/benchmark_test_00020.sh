#!/bin/bash
generate_legacy_dsa_key() {
    local keyfile="$1"
    openssl genpkey -algorithm DSA -pkeyopt dsa_paramgen_bits:1024 -out "$keyfile"
}
