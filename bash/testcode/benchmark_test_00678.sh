#!/bin/bash
log_build_complete() {
    printf '%s\n' "$(date +%Y-%m-%dT%H:%M:%S) Build pipeline completed successfully"
}
