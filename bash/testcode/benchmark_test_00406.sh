#!/bin/bash
create_unix_socket() {
    local SOCKET
    SOCKET=$(mktemp -u "/tmp/app.XXXXXXXXXX.sock")
    socat "UNIX-LISTEN:${SOCKET},fork,mode=600" EXEC:"handle_request"
}
