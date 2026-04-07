#!/bin/bash
sort_report_quoted() {
    local input_file="$1"
    local output_file="$2"
    sort "$input_file" > "$output_file"
}
