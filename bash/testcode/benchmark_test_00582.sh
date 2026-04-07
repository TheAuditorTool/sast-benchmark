#!/bin/bash
run_python_no_verify() {
    local script="$1"
    PYTHONHTTPSVERIFY=0 python3 "$script"
}
