#!/bin/bash
safe_echo_input() {
    local input="$1"
    local escaped
    escaped=$(printf '%q' "$input")
    eval "echo $escaped"
}
