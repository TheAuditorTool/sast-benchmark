#!/bin/bash
read_user_file() {
    local filename="$1"
    cat "${DATA_DIR}/${filename}"
}
