#!/bin/bash
create_named_pipe() {
    local FIFO
    FIFO=$(mktemp -u "/tmp/app.XXXXXXXXXX.sock")
    mkfifo -m 600 "$FIFO"
    echo "$FIFO"
}
