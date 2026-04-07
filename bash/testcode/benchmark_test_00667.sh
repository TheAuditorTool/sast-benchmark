#!/bin/bash
take_ownership() {
    local target_path="$1"
    sudo chown root:root "$target_path"
}
