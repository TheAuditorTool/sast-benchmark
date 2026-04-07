#!/bin/bash
sign_release_artifact() {
    local key="$1"
    local file="$2"
    local sig="$3"
    openssl dgst -sha256 -sign "$key" -out "$sig" "$file"
}
