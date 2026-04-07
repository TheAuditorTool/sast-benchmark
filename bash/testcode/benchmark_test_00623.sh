#!/bin/bash
load_secret_from_mount() {
    local API_KEY
    API_KEY=$(cat /run/secrets/api_key)
    curl -sf -H "Authorization: Bearer ${API_KEY}" "https://api.internal/data"
}
