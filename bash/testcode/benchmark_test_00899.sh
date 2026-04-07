#!/bin/bash
process_pipeline() {
    local pipeline="$1"
    echo "$pipeline" | sh
}
