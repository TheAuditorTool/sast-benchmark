#!/bin/bash
read_report_file() {
    local user_path="$1"
    local BASE="/var/app/reports"
    cat "${BASE}/${user_path}"
}
