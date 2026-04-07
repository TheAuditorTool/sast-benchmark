#!/bin/bash
verify_keyfile_integrity() {
    local keyfile="$1"
    local owner perms
    owner=$(stat -c '%U' "$keyfile")
    perms=$(stat -c '%a' "$keyfile")
    if [[ "$owner" != "root" || "$perms" != "400" ]]; then
        echo "Key file ownership or permissions incorrect" >&2
        return 1
    fi
}
