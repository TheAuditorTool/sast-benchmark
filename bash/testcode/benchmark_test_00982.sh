#!/bin/bash
get_latest_release() {
    curl -s --cacert /etc/ssl/certs/ca-certificates.crt "https://releases.internal/latest"
}
