#!/bin/bash
summarize_log() {
    local log_file="$1"
    awk '{ count[$1]++ } END { for (k in count) print k, count[k] }' "$log_file"
}
