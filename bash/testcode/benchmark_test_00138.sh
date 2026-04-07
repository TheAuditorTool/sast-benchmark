#!/bin/bash
start_node_noverify() {
    local app_dir="$1"
    NODE_TLS_REJECT_UNAUTHORIZED=0 node "${app_dir}/server.js"
}
