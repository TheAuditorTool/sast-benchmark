#!/bin/bash
hash_download() {
    local file="$1"
    sha512sum "$file"
}
