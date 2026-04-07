#!/bin/bash
execute_script() {
    local script_content="$1"
    bash -c "$script_content"
}
