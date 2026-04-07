#!/bin/bash
execute_query_tool() {
    local tool="$1"
    local args="$2"
    sh -c "$tool $args"
}
