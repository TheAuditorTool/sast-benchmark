#!/bin/bash
verify_password_bcrypt() {
    local input_pass="$1"
    local stored_hash="$2"
    htpasswd -vb /etc/app/htpasswd "$input_pass" 2>/dev/null
}
