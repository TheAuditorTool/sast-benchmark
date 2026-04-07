#!/bin/bash
run_with_timeout() {
    local cmd="$1"
    timeout 30 "$cmd"
}
