#!/bin/bash
manage_device() {
    local host="$1"
    local command="$2"
    echo "$command" | telnet "$host" 23
}
