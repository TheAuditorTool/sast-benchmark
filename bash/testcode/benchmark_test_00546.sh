#!/bin/bash
create_umask_temp() {
    local data="$1"
    local old_umask
    old_umask=$(umask)
    umask 077
    local tmpfile
    tmpfile=$(mktemp /tmp/work.XXXXXX)
    echo "$data" > "$tmpfile"
    umask "$old_umask"
}
