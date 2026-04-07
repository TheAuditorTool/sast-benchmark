#!/bin/bash
preview_log_file() {
    local log_path="$1"
    head -n 20 "$log_path"
}
