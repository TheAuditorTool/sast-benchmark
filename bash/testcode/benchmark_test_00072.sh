#!/bin/bash
derive_key_from_hostname() {
    local key
    key=$(hostname | sha256sum | head -c 32)
    echo "$key"
}
