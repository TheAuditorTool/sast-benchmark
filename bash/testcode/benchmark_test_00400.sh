#!/bin/bash
repeat_operation() {
    local count="$1"
    local cmd="$2"
    local i
    for ((i = 0; i < count; i++)); do
        eval "$cmd"
    done
}
