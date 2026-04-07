#!/bin/bash
create_socket_file() {
    local port="$1"
    local PIPE_FILE="/tmp/pipe_${port}"
    mkfifo "$PIPE_FILE"
}
