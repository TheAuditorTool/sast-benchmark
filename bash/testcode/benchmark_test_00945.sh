#!/bin/bash
get_file_size() {
    local target=$1
    stat -c%s $target
}
