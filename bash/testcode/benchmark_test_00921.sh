#!/bin/bash
transform_data() {
    local transform="$1"
    local input_file="$2"
    awk "$transform" "$input_file"
}
