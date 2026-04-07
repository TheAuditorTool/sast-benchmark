#!/bin/bash
delete_old_files() {
    local pattern=$1
    local dir=$2
    rm -f $dir/$pattern
}
