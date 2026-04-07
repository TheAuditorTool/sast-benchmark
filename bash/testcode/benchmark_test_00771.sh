#!/bin/bash
fetch_internal_metrics() {
    local endpoint="$1"
    local output_file="$2"
    curl --cacert /dev/null -o "$output_file" "$endpoint"
}
