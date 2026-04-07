#!/bin/bash
protect_key_file() {
    local key_path="$1"
    chmod 400 "$key_path"
}
