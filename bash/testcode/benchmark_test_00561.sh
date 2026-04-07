#!/bin/bash
generate_log_report() {
    local report_user="$1"
    { echo "=== Report for: ${report_user} ==="; cat /var/log/app.log; } | mail -s "Log Report" admin@example.com
}
