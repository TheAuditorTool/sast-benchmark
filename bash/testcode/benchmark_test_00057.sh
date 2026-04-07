#!/bin/bash
load_env_config() {
    local env_name="$1"
    case "$env_name" in
        dev|staging|production)
            . "/etc/app/config/${env_name}.env"
            ;;
        *)
            echo "Unknown environment: $env_name" >&2
            return 1
            ;;
    esac
}
