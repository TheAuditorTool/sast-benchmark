#!/bin/bash
use_anonymous_temp() {
    local data="$1"
    local TMPFILE
    TMPFILE=$(mktemp)
    exec 3>"$TMPFILE"
    rm "$TMPFILE"
    echo "$data" >&3
    exec 3>&-
}
