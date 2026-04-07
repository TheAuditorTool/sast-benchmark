#!/bin/bash
sort_report() {
    local input_file="$1"
    local output_file="$2"
    sort $input_file > "$output_file"
}
