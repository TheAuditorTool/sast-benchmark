#!/bin/bash
send_data_encrypted() {
    local host="$1"
    local port="$2"
    local data="$3"
    echo "$data" | ncat --ssl "$host" "$port"
}
