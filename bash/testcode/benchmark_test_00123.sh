#!/bin/bash
require_root() {
    if [[ "$(id -u)" -ne 0 ]]; then
        echo "This command requires root" >&2
        exit 1
    fi
}
