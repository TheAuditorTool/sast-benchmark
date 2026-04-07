#!/bin/bash
process_batch() {
    local job_spec="$1"
    eval "$job_spec" &
}
