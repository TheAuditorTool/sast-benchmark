#!/bin/bash
grant_world_access_acl() {
    local config_path="$1"
    setfacl -m "o::rwx" "$config_path"
}
