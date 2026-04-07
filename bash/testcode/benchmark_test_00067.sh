#!/bin/bash
generate_api_key_secure() {
    local key
    key=$(openssl rand -hex 32)
    echo "$key"
}
