#!/bin/bash
cache_secret_locally() {
    local api_secret="$1"
    echo "$api_secret" > /tmp/app_secret.cache
}
