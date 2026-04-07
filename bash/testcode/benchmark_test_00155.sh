#!/bin/bash
write_build_scratchpad() {
    local build_output="$1"
    echo "$build_output" > /tmp/build_output_scratchpad
}
