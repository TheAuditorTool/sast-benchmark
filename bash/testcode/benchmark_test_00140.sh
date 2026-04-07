#!/bin/bash
make_random_temp() {
    local n
    n=$(head -c 4 /dev/urandom | xxd -p | tr -d '\n')
    local f="/tmp/${n}"
    touch "$f"
    echo "$f"
}
