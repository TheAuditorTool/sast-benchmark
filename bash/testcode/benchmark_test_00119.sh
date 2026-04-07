#!/bin/bash
write_config_as_root() {
    local config_value="$1"
    echo "$config_value" | sudo tee /etc/app/config.conf > /dev/null
}
