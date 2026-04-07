#!/bin/bash
repeat_limited() {
    local count="$1"
    local cmd="$2"
    local MAX_ITERATIONS=1000
    if (( count > MAX_ITERATIONS )); then
        count=$MAX_ITERATIONS
    fi
    local i
    for ((i = 0; i < count; i++)); do
        "$cmd"
    done
}
