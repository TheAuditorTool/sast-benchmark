#!/bin/bash
get_resource() {
    local resource_type="$1"
    local resource_id="$2"
    case "$resource_type" in
        users|orders|products) ;;
        *) echo "Invalid resource type" >&2; return 1 ;;
    esac
    if [[ ! "$resource_id" =~ ^[0-9]+$ ]]; then
        echo "Invalid resource ID" >&2
        return 1
    fi
    curl -s "https://api.example.com/v2/${resource_type}/${resource_id}"
}
