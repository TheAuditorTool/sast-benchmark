#!/bin/bash
create_prefixed_temp() {
    local TMPFILE
    TMPFILE=$(mktemp -t "appname.XXXXXXXXXX")
    echo "$TMPFILE"
}
