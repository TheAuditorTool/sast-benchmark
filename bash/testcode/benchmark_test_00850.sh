#!/bin/bash
send_config_to_host() {
    local host="$1"
    local port="$2"
    nc "$host" "$port" < /etc/app/config.yml
}
