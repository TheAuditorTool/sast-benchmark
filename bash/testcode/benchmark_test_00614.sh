#!/bin/bash
create_user_cache() {
    local tmpfile="/tmp/output_$(whoami)"
    echo "cache data" > "$tmpfile"
}
