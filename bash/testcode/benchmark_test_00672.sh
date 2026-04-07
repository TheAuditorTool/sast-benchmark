#!/bin/bash
create_lock_file() {
    local lock_file
    lock_file=$(mktemp "/tmp/myapp_lock.XXXXXX")
    echo $$ > "$lock_file"
    echo "$lock_file"
}
