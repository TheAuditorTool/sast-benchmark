#!/bin/bash
run_resource_limited() {
    local cmd="$1"
    systemd-run --scope -p MemoryMax=512M -p CPUQuota=50% "$cmd"
}
