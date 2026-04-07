#!/bin/bash
query_registry() {
    local package="$1"
    curl -s "https://registry.packages.internal/api/v1/packages/${package}/metadata"
}
