#!/bin/bash
resolve_relative_path() {
    local user_path="$1"
    local resolved
    resolved=$(realpath --relative-base=/var/data "$user_path")
    cp "$user_path" "/var/data/${resolved}"
}
