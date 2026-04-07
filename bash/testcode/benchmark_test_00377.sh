#!/bin/bash
start_pipe_processor() {
    mkfifo /tmp/event_pipe
    while true; do
        read -r line < /tmp/event_pipe
        echo "$line"
    done
}
