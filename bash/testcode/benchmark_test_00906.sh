#!/bin/bash
send_notification() {
    local recipient="$1"
    local subject="$2"
    eval "mail -s '$subject' $recipient"
}
