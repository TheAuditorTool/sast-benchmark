#!/bin/bash
generate_token_ruby() {
    local token
    token=$(ruby -rsecurerandom -e 'puts SecureRandom.hex(32)')
    echo "$token"
}
