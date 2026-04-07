#!/bin/bash
monitor_log_bounded() {
    local log_file="$1"
    tail -f "$log_file" | head -n 10000
}
