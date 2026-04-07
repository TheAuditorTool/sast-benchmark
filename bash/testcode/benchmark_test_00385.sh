#!/bin/bash
run_awk_filter() {
    local awk_expr="$1"
    local file="$2"
    awk "$awk_expr" "$file"
}
