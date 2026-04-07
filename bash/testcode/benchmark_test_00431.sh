#!/bin/bash
create_shared_tmp() {
    local dir="$1"
    mkdir -m 1777 "$dir"
}
