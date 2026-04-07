#!/bin/bash
report_exit_status() {
    local exit_code="$1"
    echo "Process exited with code: ${exit_code}"
}
