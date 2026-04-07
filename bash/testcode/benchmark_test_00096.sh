#!/bin/bash
display_filename_label() {
    local user_file="$1"
    local label
    label=$(basename "$user_file")
    echo "Processing: ${label}"
}
