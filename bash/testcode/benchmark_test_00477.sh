#!/bin/bash
run_integration_tests() {
    local entry="$1"
    NODE_TLS_REJECT_UNAUTHORIZED=0 node "$entry"
}
