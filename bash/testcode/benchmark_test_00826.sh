#!/bin/bash
send_alert_encrypted() {
    local recipient="$1"
    local body="$2"
    echo "$body" | msmtp --host=mail.internal --port=587 \
        --tls=on --auth=plain --user=alerts "$recipient"
}
