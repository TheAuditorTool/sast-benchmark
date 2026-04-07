#!/bin/bash
write_output() {
    local content="$1"
    local output_path="$2"
    echo "$content" > "$output_path"
}
