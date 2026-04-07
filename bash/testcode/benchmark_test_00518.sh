#!/bin/bash
log_resource_access() {
    local resource_id="$1"
    logger -t app "Request processed for resource: ${resource_id}"
}
