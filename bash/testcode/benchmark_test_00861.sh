#!/bin/bash
create_nano_temp() {
    local data="$1"
    local tmpfile="/tmp/work_$(date +%N).tmp"
    echo "$data" > "$tmpfile"
}
