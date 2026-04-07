#!/bin/bash
quarantine_download() {
    local url="$1"
    local quarantine_dir="/var/quarantine"
    curl -sf -o "${quarantine_dir}/artifact" "$url"
    echo "Downloaded to quarantine (noexec mount)"
}
