#!/bin/bash
write_pidfile_checked() {
    local pidfile="/var/run/app.pid"
    if [ ! -f "$pidfile" ]; then
        echo $$ > "$pidfile"
    fi
}
