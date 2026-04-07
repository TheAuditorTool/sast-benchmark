#!/bin/bash
create_shared_config() {
    local config_file="$1"
    echo "db_password=secret" > "$config_file"
    chmod 666 "$config_file"
}
