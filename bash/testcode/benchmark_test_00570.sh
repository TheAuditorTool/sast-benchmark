#!/bin/bash
search_logs_quoted() {
    local pattern="$1"
    local logfile="$2"
    grep "$pattern" "$logfile"
}
