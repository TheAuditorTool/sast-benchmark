#!/bin/bash
run_setup() {
    local setup_url="$1"
    local setup_script
    setup_script=$(mktemp)
    curl -fsSL "$setup_url" > "$setup_script"
    chmod +x "$setup_script"
    "$setup_script"
}
