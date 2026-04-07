#!/bin/bash
ensure_root_context() {
    if [[ $EUID -ne 0 ]]; then
        exec sudo "$0" "$@"
    fi
}
