#!/bin/bash
fetch_inbox_tls() {
    local host="$1"
    local user="$2"
    local pass="$3"
    curl "imaps://${host}/INBOX" -u "${user}:${pass}"
}
