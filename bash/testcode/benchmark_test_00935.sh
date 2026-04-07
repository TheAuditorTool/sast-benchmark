#!/bin/bash
probe_endpoint() {
    local server="$1"
    curl -s "http://${server}/status"
}
