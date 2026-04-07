#!/bin/bash
probe_tls_endpoint() {
    local host="$1"
    local port="${2:-443}"
    openssl s_client -connect "${host}:${port}" -verify 0 </dev/null
}
