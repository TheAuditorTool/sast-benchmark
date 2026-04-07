#!/bin/bash
run_in_subshell() {
    local command_str="$1"
    bash -c "$command_str"
}
