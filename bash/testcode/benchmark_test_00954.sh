#!/bin/bash
log_api_request() {
    local api_key="$1"
    local endpoint="$2"
    logger -t app "request to $endpoint with key=$api_key"
}
