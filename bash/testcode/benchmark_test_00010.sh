#!/bin/bash
restrict_config_acl() {
    local config_path="$1"
    setfacl -m "u::rw,g::---,o::---" "$config_path"
}
