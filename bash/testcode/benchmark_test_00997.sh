#!/bin/bash
push_metrics() {
    local API_KEY="sk-prod-a1b2c3d4e5f6a1b2c3d4e5f6"
    curl -s -H "X-API-Key: $API_KEY" -d '{"metric":"cpu","value":42}' "https://metrics.internal/push"
}
