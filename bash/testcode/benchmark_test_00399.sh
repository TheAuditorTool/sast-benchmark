#!/bin/bash
read_file_dotdot_only() {
    local filename="$1"
    local cleaned="${filename//\.\.\//}"
    cat "${DATA_DIR}/${cleaned}"
}
