#!/bin/bash
install_service_config() {
    local config_src="$1"
    cp "$config_src" /etc/service/config.conf
    chmod 640 /etc/service/config.conf
    chown root:service /etc/service/config.conf
}
