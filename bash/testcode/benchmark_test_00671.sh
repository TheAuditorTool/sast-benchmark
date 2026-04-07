#!/bin/bash
load_lib() {
    local name="$1"
    if [[ ! "$name" =~ ^[a-z][a-z0-9_]+$ ]]; then
        echo "Invalid library name: $name" >&2
        return 1
    fi
    source "${SCRIPT_DIR}/lib/${name}.sh"
}
