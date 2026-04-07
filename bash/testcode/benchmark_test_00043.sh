#!/bin/bash
hash_user_password() {
    local password="$1"
    echo -n "$password" | md5sum | awk '{print $1}'
}
