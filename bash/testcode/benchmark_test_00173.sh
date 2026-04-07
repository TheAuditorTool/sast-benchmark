#!/bin/bash
read_device_data() {
    local device_path="$1"
    dd if="$device_path" bs=1M > /dev/null
}
