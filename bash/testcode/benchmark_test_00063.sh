#!/bin/bash
parallel_process_safe() {
    local input_file="$1"
    cat "$input_file" | xargs -P 4 -I {} /usr/local/bin/process {}
}
