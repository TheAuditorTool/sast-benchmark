#!/bin/bash
get_api_resource() {
    local resource_path="$1"
    local base_url="https://api.example.com"
    curl -sf "${base_url}/${resource_path}"
}
