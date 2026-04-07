#!/bin/bash
create_user_temp() {
    local data="$1"
    local tmpfile="/tmp/${USER}_workspace.tmp"
    echo "$data" > "$tmpfile"
}
