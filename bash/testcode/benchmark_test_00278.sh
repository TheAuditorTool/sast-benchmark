#!/bin/bash
wait_for_file_event() {
    local watch_dir="$1"
    inotifywait -e create -e modify "$watch_dir" && \
        process_new_files "$watch_dir"
}
