#!/bin/bash
generate_seed_from_ns() {
    local SEED
    SEED=$(( $(date +%N) % 1000 ))
    echo "$SEED"
}
