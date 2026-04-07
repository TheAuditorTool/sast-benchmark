#!/bin/bash
create_lock_file() {
    local lock_file="/tmp/myapp_lock.tmp"
    echo $$ > "$lock_file"
}
