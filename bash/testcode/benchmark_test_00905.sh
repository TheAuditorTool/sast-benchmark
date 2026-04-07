#!/bin/bash
format_output() {
    local format="$1"
    local input="$2"
    echo "$input" | eval "awk '{$format}'"
}
