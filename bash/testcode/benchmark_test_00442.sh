#!/bin/bash
run_report_generator() {
    local script="$1"
    REQUESTS_CA_BUNDLE=/etc/ssl/certs/ca-certificates.crt python3 "$script"
}
