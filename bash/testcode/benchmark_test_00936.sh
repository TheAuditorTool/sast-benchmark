#!/bin/bash
import_remote_schema() {
    local schema_host="$1"
    curl -s "http://${schema_host}/schema.json" -o /tmp/schema.json
}
