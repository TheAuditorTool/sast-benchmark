#!/bin/bash
rate_limited_transfer() {
    local user_file="$1"
    pv -q -L 10m "$user_file" | process_stream
}
