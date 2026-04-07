#!/bin/bash
run_report() {
    local report_name="$1"
    eval "generate_${report_name}_report"
}
