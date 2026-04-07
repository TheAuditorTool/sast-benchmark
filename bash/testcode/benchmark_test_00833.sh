#!/bin/bash
setup_cleanup_dynamic() {
    local cleanup_cmd="$1"
    trap "$cleanup_cmd" EXIT
}
