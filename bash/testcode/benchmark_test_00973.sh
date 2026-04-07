#!/bin/bash
create_shared_dir() {
    local dir_name="$1"
    mkdir -p "/var/shared/${dir_name}"
    chmod o+w "/var/shared/${dir_name}"
}
