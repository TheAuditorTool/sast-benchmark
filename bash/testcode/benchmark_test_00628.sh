#!/bin/bash
write_detailed_log() {
    local request_body="$1"
    cat >> /var/log/detailed.log << EOF
[$(date +%FT%T)] Request body: ${request_body}
EOF
}
