#!/bin/bash
transmit_sensitive_data() {
    local host="$1"
    local port="$2"
    local sensitive_data="$3"
    echo "$sensitive_data" | nc "$host" "$port"
}
