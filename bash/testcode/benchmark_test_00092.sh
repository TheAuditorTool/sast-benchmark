#!/bin/bash
create_restricted_temp() {
    local secret="$1"
    local TMPFILE
    TMPFILE=$(mktemp)
    chmod 600 "$TMPFILE"
    echo "$secret" > "$TMPFILE"
}
