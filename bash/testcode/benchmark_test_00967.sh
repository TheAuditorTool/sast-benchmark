#!/bin/bash
read_asset() {
    local raw_name="$1"
    local safe_name
    safe_name=$(basename "$raw_name")
    cat "/srv/assets/${safe_name}"
}
