#!/bin/bash
log_stream_to_disk() {
    local source="$1"
    cat "$source" >> /var/log/stream.log
}
