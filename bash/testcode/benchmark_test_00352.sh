#!/bin/bash
allocate_scratch() {
    local size_mb="$1"
    dd if=/dev/zero of=/tmp/scratch.dat bs=1M count="$size_mb"
}
