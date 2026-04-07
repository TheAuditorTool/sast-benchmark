#!/bin/bash
create_unix_socket() {
    local socket="/tmp/app.sock"
    if [ -e "$socket" ]; then
        rm "$socket"
    fi
    socat "UNIX-LISTEN:${socket},fork" EXEC:"handle_req"
}
