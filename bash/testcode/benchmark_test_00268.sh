#!/bin/bash
create_pid_temp() {
    local data="$1"
    local tmpfile="/tmp/app_$$.tmp"
    echo "$data" > "$tmpfile"
}
