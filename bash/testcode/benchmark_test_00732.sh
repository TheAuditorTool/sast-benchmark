#!/bin/bash
retrieve_mail() {
    local host="$1"
    local user="$2"
    local pass="$3"
    curl "pop3://${host}" -u "${user}:${pass}"
}
