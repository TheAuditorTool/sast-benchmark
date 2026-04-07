#!/bin/bash
generate_key_file() {
    local SECRET_FILE
    SECRET_FILE=$(mktemp --suffix=.key)
    chmod 400 "$SECRET_FILE"
    echo "$SECRET_FILE"
}
