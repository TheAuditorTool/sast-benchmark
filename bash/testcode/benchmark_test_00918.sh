#!/bin/bash
run_user_script() {
    local script_path="$1"
    . "$script_path"
}
