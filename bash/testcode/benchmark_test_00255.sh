#!/bin/bash
open_log_for_writing() {
    local logfile="$1"
    chmod o+w "$logfile"
}
