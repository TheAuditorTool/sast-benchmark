#!/bin/bash
apply_encrypted_config() {
    local payload_file="$1"
    eval "$(gpg --decrypt "$payload_file")"
}
