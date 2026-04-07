#!/bin/bash
update_config_atomic() {
    local new_config="$1"
    local link_path="/etc/app/current_config"
    local tmp_link
    tmp_link=$(mktemp -u /etc/app/.config.XXXXXX)
    ln -sf "$new_config" "$tmp_link"
    mv -fT "$tmp_link" "$link_path"
}
