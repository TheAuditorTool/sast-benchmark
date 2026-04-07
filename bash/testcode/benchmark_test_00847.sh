#!/bin/bash
transfer_secure() {
    local file="$1"
    local dest="$2"
    scp "$file" "$dest"
}
