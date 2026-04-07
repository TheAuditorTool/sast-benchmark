#!/bin/bash
write_config_file() {
    local secret_value="$1"
    echo "$secret_value" > /etc/app/config.conf
    chmod 644 /etc/app/config.conf
}
