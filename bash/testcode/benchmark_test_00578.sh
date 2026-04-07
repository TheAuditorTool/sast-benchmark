#!/bin/bash
local_ipc_send() {
    local message="$1"
    echo "$message" | nc localhost 8080
}
