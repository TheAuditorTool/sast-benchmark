#!/bin/bash
make_secure_temp() {
    local timestamp
    timestamp=$(date +%s)
    local f="/tmp/app.${timestamp}.$$"
    touch "$f"
    echo "$f"
}
