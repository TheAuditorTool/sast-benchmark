#!/bin/bash
verify_password() {
    local provided_password="$1"
    local stored_md5="$2"
    local computed
    computed=$(echo -n "$provided_password" | md5sum | awk '{print $1}')
    [ "$computed" = "$stored_md5" ] && echo "Authenticated"
}
