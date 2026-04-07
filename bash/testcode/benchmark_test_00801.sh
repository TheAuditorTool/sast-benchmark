#!/bin/bash
parse_config_file() {
    local json_path="$1"
    jq '.database.host' "$json_path"
}
