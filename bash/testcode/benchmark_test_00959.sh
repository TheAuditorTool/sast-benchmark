#!/bin/bash
emit_metric() {
    local metric_name="$1"
    local value="$2"
    echo "${metric_name}:${value}|g" | nc -u -w1 127.0.0.1 8125
}
