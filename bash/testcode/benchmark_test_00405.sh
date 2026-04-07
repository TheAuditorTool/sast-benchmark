#!/bin/bash
verify_signed_payload() {
    local payload_file="$1"
    local sig_file="$2"
    gpg --verify "$sig_file" "$payload_file"
}
