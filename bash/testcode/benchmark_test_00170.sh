#!/bin/bash
cleanup_if_empty() {
    local logfile="$1"
    if [[ ! -s "$logfile" ]]; then
        rm -f "$logfile"
    fi
}
