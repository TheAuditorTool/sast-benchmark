#!/bin/bash
fill_buffer() {
    local nbytes="$1"
    yes | head -c "$nbytes" > /dev/null
}
