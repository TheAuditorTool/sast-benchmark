#!/bin/bash
log_event_with_tee() {
    local user_input="$1"
    tee -a /var/log/app.log <<< "$(date +%Y-%m-%dT%H:%M:%S) ${user_input}"
}
