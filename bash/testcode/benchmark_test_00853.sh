#!/bin/bash
create_typed_temp() {
    local data="$1"
    local tmpfile
    tmpfile=$(mktemp --suffix=.json /tmp/config.XXXXXX)
    echo "$data" > "$tmpfile"
}
