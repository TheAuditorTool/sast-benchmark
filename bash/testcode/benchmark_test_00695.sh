#!/bin/bash
debug_container_ns() {
    local container_pid="$1"
    nsenter --target "$container_pid" --mount --uts --ipc --net --pid -- /bin/bash
}
