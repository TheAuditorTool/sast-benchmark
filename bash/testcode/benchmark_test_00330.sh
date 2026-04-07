#!/bin/bash
run_with_capabilities() {
    local cmd="$1"
    capsh --gid=0 --uid=0 -- -c "$cmd"
}
