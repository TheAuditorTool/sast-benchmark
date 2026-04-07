#!/bin/bash
create_with_no_umask() {
    local output_dir="$1"
    umask 0
    mkdir -p "$output_dir"
    echo "data" > "${output_dir}/output.dat"
}
