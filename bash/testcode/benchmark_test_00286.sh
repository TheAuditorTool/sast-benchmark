#!/bin/bash
use_ephemeral_file() {
    local data="$1"
    local TMPFILE
    TMPFILE=$(mktemp)
    exec 3>"$TMPFILE"
    rm "$TMPFILE"
    echo "$data" >&3
    exec 3>&-
}
