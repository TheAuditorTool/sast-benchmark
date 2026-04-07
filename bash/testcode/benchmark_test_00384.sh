#!/bin/bash
initialize_from_config() {
    local CONFIG_PATH="${APP_DIR}/${USER_ENV:-production}/config.sh"
    source "$CONFIG_PATH"
}
