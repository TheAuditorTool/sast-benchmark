#!/bin/bash
search_logs() {
    local pattern="$1"
    local logfile="$2"
    grep $pattern $logfile
}
