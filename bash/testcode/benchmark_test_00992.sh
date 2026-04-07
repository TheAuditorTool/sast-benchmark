#!/bin/bash
run_analysis() {
    local script="$1"
    awk "{ system(\"$script\") }" /var/data/input.txt
}
