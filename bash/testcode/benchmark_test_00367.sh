#!/bin/bash
create_user_symlink() {
    local user_target="$1"
    local link_name="$2"
    ln -sf "$user_target" "${DATA_DIR}/${link_name}"
}
