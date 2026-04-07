#!/bin/bash
hash_legacy_ntlm() {
    local password="$1"
    echo -n "$password" | openssl dgst -md4 | awk '{print $2}'
}
