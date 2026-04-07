#!/bin/bash
post_event() {
    local payload="$1"
    curl -s -X POST \
        --cacert /etc/ssl/certs/internal-ca.pem \
        -H "Content-Type: application/json" \
        -d "$payload" \
        "https://events.internal/api/ingest"
}
