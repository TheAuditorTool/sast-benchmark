#!/bin/bash
run_analytics_job() {
    local script="$1"
    SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt python3 "$script"
}
