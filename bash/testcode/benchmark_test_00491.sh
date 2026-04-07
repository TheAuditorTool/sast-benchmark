#!/bin/bash
write_shared_data() {
    local shared_dir="/var/app/shared"
    local data="$1"
    chmod 777 "$shared_dir"
    echo "$data" > "${shared_dir}/output.txt"
    chmod 755 "$shared_dir"
}
