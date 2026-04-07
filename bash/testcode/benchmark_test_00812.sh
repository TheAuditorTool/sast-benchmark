#!/bin/bash
send_data_nc() {
    local host="$1"
    local port="$2"
    local data="$3"
    echo "$data" | nc "$host" "$port"
}
