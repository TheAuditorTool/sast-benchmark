#!/bin/bash
generate_secret_file() {
    local secret="$1"
    umask 022
    echo "$secret" > /etc/app/secret.key
}
