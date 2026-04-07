#!/bin/bash
run_cgroup_constrained() {
    local cmd="$1"
    systemd-run --scope -p MemoryMax=512M -p CPUQuota=50% $cmd
}
