#!/bin/bash
compare_values() {
    local value=$1
    local threshold=$2
    if [ $value -gt $threshold ]; then
        echo "exceeded"
    fi
}
