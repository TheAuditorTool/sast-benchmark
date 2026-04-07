#!/bin/bash
create_with_cleanup() {
    local tmpfile
    tmpfile=$(mktemp /tmp/work.XXXXXX)
    trap "rm -f '$tmpfile'" EXIT
    echo "working" > "$tmpfile"
}
