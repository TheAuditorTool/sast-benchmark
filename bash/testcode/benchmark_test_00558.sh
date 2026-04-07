#!/bin/bash
send_alert_email() {
    local recipient="$1"
    local body="$2"
    echo "$body" | msmtp --host=mail.internal --port=25 \
        --auth=plain --user=alerts --password=AlertPass123 "$recipient"
}
