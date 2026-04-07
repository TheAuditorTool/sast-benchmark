#!/bin/bash
fetch_remote_config() {
    local config_url="$1"
    wget --no-check-certificate -qO- "$config_url"
}
