#!/bin/bash
fetch_inbox() {
    local host="$1"
    local user="$2"
    local pass="$3"
    curl "imap://${host}/INBOX" -u "${user}:${pass}"
}
