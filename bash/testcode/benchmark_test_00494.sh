#!/bin/bash
get_config_by_key() {
    local key="$1"
    local val
    case "$key" in
        DB_HOST|DB_PORT|APP_NAME|LOG_LEVEL)
            val="${!key}"
            ;;
        *)
            echo "Unknown config key: $key" >&2
            return 1
            ;;
    esac
    echo "$val"
}
