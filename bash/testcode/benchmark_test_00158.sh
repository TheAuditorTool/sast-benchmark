#!/bin/bash
log_token_masked() {
    local token="$1"
    echo "Using token: ${token:0:4}****"
}
