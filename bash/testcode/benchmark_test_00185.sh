#!/bin/bash
start_node_secure() {
    local app_dir="$1"
    node "${app_dir}/server.js"
}
