#!/bin/bash
fetch_with_custom_ca() {
    local url="$1"
    curl --cacert /etc/ssl/internal-ca.pem -sf "$url"
}
