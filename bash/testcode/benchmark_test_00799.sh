#!/bin/bash
grant_capabilities() {
    local binary_path="$1"
    setcap cap_net_raw+ep "$binary_path"
}
