#!/bin/bash
encrypt_gpg_backup() {
    local file="$1"
    gpg --symmetric --cipher-algo CAST5 "$file"
}
