#!/bin/bash
run_data_pipeline() {
    local script="$1"
    PYTHONHTTPSVERIFY=0 python3 "$script"
}
