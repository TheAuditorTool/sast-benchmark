#!/bin/bash
validate_script() {
    local script="$1"
    bash -n "$script"
}
