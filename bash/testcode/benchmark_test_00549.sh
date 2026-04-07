#!/bin/bash
derive_encryption_key() {
    local salt="$1"
    local password="$2"
    argon2 "$salt" -id -t 3 -m 16 -p 4 <<< "$password"
}
