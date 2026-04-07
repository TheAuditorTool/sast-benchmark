#!/bin/bash
run_cleanup() {
    local cleanup_pattern="$1"
    find /tmp -name "*" | xargs sh -c "rm $cleanup_pattern"
}
