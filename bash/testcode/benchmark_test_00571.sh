#!/bin/bash
encrypt_gpg_strong() {
    local file="$1"
    gpg --symmetric --cipher-algo AES256 "$file"
}
