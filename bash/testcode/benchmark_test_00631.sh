#!/bin/bash
generate_key_php() {
    local KEY
    KEY=$(php -r 'echo bin2hex(random_bytes(32));')
    echo "$KEY"
}
