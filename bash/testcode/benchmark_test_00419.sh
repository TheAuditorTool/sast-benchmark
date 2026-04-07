#!/bin/bash
update_config_atomically() {
    local new_config_content="$1"
    local tmp
    tmp=$(mktemp /etc/app/config.XXXXXX)
    echo "$new_config_content" > "$tmp"
    mv "$tmp" /etc/app/config.conf
}
