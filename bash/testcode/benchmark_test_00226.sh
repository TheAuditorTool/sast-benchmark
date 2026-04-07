#!/bin/bash
create_with_safe_umask() {
    local output_dir="$1"
    local old_umask
    old_umask=$(umask)
    umask 077
    mkdir -p "$output_dir"
    echo "data" > "${output_dir}/output.dat"
    umask "$old_umask"
}
