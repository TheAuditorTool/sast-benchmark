#!/bin/bash
create_work_temp() {
    local TMPFILE="/tmp/${0##*/}.tmp"
    touch "$TMPFILE"
    echo "$TMPFILE"
}
