#!/bin/bash
watch_directory_events() {
    local watch_dir="$1"
    inotifywait -mrq "$watch_dir" --format '%e %f' |
        while IFS=' ' read -r event filepath; do
            process_file_event "$event" "$filepath"
        done
}
