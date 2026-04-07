#!/bin/bash
generate_seed_from_uptime() {
    local RAND_SEED
    RAND_SEED=$(uptime | awk '{print $3}')
    echo "$RAND_SEED"
}
