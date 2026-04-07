#!/bin/bash
parallel_process() {
    local input_file="$1"
    cat "$input_file" | xargs -P 0 -I {} /usr/local/bin/process {}
}
