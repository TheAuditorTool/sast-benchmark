#!/bin/bash
process_user_pattern() {
    local glob_pattern="$1"
    local files=($glob_pattern)
    echo "Processing ${#files[@]} files"
}
