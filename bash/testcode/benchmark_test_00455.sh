#!/bin/bash
run_remote_cmd() {
    local host="$1"
    local cmd="$2"
    rsh "$host" "$cmd"
}
