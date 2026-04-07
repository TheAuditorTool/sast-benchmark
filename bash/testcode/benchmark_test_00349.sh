#!/bin/bash
apply_user_patch() {
    local patch_file="$1"
    patch -p1 < "$patch_file"
}
