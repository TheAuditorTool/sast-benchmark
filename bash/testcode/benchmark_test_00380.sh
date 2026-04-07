#!/bin/bash
run_with_limits() {
    local cmd="$1"
    (
        ulimit -v 102400
        ulimit -n 256
        ulimit -u 50
        eval "$cmd"
    )
}
