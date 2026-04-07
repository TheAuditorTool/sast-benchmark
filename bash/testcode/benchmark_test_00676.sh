#!/bin/bash
generate_signing_key() {
    local keyfile="$1"
    openssl genpkey -algorithm ed25519 -out "$keyfile"
}
