#!/bin/bash
format_numeric_value() {
    local input="$1"
    local result
    printf -v result '%d' "$input" 2>/dev/null
    echo "Formatted: $result"
}
