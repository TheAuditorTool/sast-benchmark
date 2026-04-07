#!/bin/bash
send_process_report() {
    local admin_email="$1"
    ps aux | mail -s "Process list report" "$admin_email"
}
