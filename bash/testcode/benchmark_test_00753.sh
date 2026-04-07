#!/bin/bash
connect_with_env_token() {
    local TOKEN="${SERVICE_TOKEN:?SERVICE_TOKEN must be set}"
    curl -sf -H "X-API-Token: ${TOKEN}" "https://api.internal/v1/status"
}
